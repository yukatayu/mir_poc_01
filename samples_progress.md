# samples_progress

Last updated: 2026-05-21 19:56 JST

Current repo-local focus: current-L2 runnable floor, practical alpha-1 evidence, bounded operational α-0.5 / α-0.8 / α-0.9 workflows, product alpha-1 release candidate, installed-binary adoption probe, canonical operational product sample suite, and the implementation-half queue after front-half actualization of `P-COMP-01` / `P-POSE-01` / `P-PROJ-01` / `P-ENG-01`. `samples/alpha/` remains alpha-0 evidence; `samples/practical-alpha1/` remains first-floor fixture evidence; `samples/product-alpha1/` is the current product alpha root. Docker skip paths are partial local probes, not release-candidate evidence.

## Legend

Primary metric:

- `workflow-ready`: an external developer can reproduce the named layer workflow end-to-end from repo commands.
- `evidence-closed`: helper / sidecar / report / expected JSON / first-floor runner evidence is synchronized and validated, but the row is not operational workflow completion.
- `boundary-fixed`: normative specs / roadmap define the boundary, but no reproducible workflow is present yet.
- `product-release-candidate`: alpha CLI can reproduce the product alpha command family through `demo` and release check, but final public product claims remain out of scope.
- `planned`: source or roadmap exists, but no reproducible workflow is present.

Notes:

- `100%` is not used for helper / sidecar / report / expected JSON / first-floor runner rows.
- Use `100%` only when a layer is externally usable as a reproducible operational workflow or product/public layer.
- helper-local preview, report-local inventory, and generated bridge evidence are not final public API.

## Workflow and Product-Boundary Snapshot

| Line | Workflow status | Reproducible command | Current evidence | Missing actualization |
|---|---|---|---|---|
| α-0.5 local observable runtime | workflow-ready: local session workflow | `python3 scripts/practical_alpha05_session.py check-all --format json` | same-session carrier, typed host-I/O `AddOne`, local observe/save/load evidence | none within bounded α-0.5 workflow |
| α-0.8 same-session hot-plug runtime | workflow-ready: same-session hot-plug workflow | `python3 scripts/practical_alpha08_session_hotplug.py check-all --format json` | debug/auth/rate-limit/object/avatar attach rows and lifecycle export | accepted detach execution / distributed ordering |
| α-0.9 session-bound devtools | workflow-ready: session-bound devtools workflow | `python3 scripts/practical_alpha09_devtools.py check-all --format json` | event DAG, route, membership, witness, hot-plug, fallback, save-load, redacted view, retention panels | final viewer/telemetry ABI, durable audit |
| practical α-1 integrated workflow | bounded workflow-ready, not product/public-ready | `python3 scripts/practical_alpha1_integrated_workflow.py check-all --format json` | front-door / checker / runtime / host-I/O / hot-plug / save-load / devtools / preview evidence in one workflow | product/public-ready α-1, final public viewer/telemetry ABI, distributed durable save/load |
| product alpha-1 release candidate | product-release-candidate, not final product | `python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release` | `mirrorea-alpha` command family, `samples/product-alpha1/demo`, local/Docker transport, non-final viewer, R0/R2 save, native host bundle, CLI `demo`, release check | user/final broader distribution decision |
| installed-binary adoption probe | bounded public-ish adoption probe | `python3 scripts/product_alpha1_installed_binary_check.py --format json check-all --out /tmp/mirrorea-alpha1-installed-binary-check` | built `target/debug/mirrorea-alpha`, generated host bundle, bundle `run.sh check/view`, `compatibility_scope`, `shipped_surface`, `distribution_scope` | archive / installer / system package / auto-update / hosted service |
| operational product sample suite | workflow-ready canonical suite, not final product | `python3 scripts/operational_product_samples.py check-all --format json` | six operational roots, shared attach packages, projection inventory, retained blueprints, authoring starters, portal/shard/gradient runtime cuts, helper scope blocks | user-spec-required broader distribution / final catalog decision |
| Mir computational core | boundary-fixed, planned scaffold actualized | `python3 scripts/mir_computational_samples.py check-all --format json` | `specs/28` / `plan/53`, `samples/product-alpha1/computational/`, `matrix.json`, and helper/unit test now classify planned rows and reject `run` as `planned_only` | no Mir-owned runtime execution yet; `P-COMP-02..04` remain |
| Transform / PoseGraph | boundary-fixed, planned scaffold actualized | `python3 scripts/posegraph_samples.py check-all --format json` | `specs/29` / `plan/54`, `samples/product-alpha1/posegraph/`, `matrix.json`, and helper/unit test now classify planned rows and reject `run` as `planned_only` | no PoseGraph runtime evidence yet; `P-POSE-02` remains |
| projection/backend boundary | boundary-fixed, planned scaffold actualized | `python3 scripts/projection_boundary_samples.py check-all --format json` | `specs/30` / `plan/55`, `samples/product-alpha1/projection/`, `matrix.json`, and helper/unit test now classify planned rows and reject `run` as `planned_only` | no projection codegen, server/client binary split, or backend execution yet |
| engine/WASM/FFI adapter boundary | boundary-fixed, planned scaffold actualized | `python3 scripts/engine_adapter_boundary_samples.py check-all --format json` | `specs/31` / `plan/56`, `samples/product-alpha1/engine-adapter/`, `matrix.json`, and helper/unit test now classify planned provider rows and reject `run` as `planned_only` | no engine integration, final FFI ABI, or admitted native/WASM execution yet |
| autonomous execution contract | boundary-fixed, no sample claim | docs validation plus package helpers | `specs/32` / `plan/57` define front-half closeout, implementation half, package cadence, and close protocol | continue `P-COMP-02..04` / `P-POSE-02` implementation packages |

## Product Alpha Root Status

| Root | Role | Runnable anchor | Current reading |
|---|---|---|---|
| `samples/product-alpha1/demo/` | release-candidate product demo | `python3 scripts/product_alpha1_release_check.py --format json check-all --out <dir>` | workflow-ready alpha demo; not final product |
| `samples/product-alpha1/operational/` | canonical operational sample suite | `python3 scripts/operational_product_samples.py check-all --format json` | workflow-ready operational suite; not final product |
| `samples/product-alpha1/operational/templates/` | template-only authoring starters | `cargo run -q -p mirrorea-cli -- check <template> --format json` | `world-core`, `membership-chat`, `sugoroku-world` starters only |
| `samples/product-alpha1/operational/future/` | future boundary inventory | JSON validation / docs references | retained blueprint/profile inventory; non-executable unless paired with active roots |
| `samples/product-alpha1/computational/` | planned Mir-owned computation roots | `python3 scripts/mir_computational_samples.py check-all --format json` | planned-only scaffold actualized; representative `.mir` files and `matrix.json` exist, but no executable runtime row yet |
| `samples/product-alpha1/posegraph/` | planned Transform / PoseGraph roots | `python3 scripts/posegraph_samples.py check-all --format json` | planned-only scaffold actualized; representative `.mir` files and `matrix.json` exist, but no runtime row yet |
| `samples/product-alpha1/projection/` | planned projection boundary roots | `python3 scripts/projection_boundary_samples.py check-all --format json` | planned-only scaffold actualized; representative inventory JSON files and `matrix.json` exist, but no codegen/runtime row yet |
| `samples/product-alpha1/engine-adapter/` | planned engine/provider boundary roots | `python3 scripts/engine_adapter_boundary_samples.py check-all --format json` | planned-only scaffold actualized; representative contract JSON files and `matrix.json` exist, but no admitted provider row yet |

## Practical Alpha-1 First-Floor Map

| Family | Classification | Validation anchor | Current reading |
|---|---|---|---|
| `SRC-01..05` | first-floor evidence | `cargo test -p mir-ast practical_alpha1_front_door -- --nocapture` | limited `package.mir.json` front-door; final grammar ではない |
| `CHK-*` | first-floor evidence | `python3 scripts/practical_alpha1_check.py check-all --format json` | checker obligations and rejected rows |
| `RUN-*` | first-floor evidence | `python3 scripts/practical_alpha1_run_local.py check-all --format json` | first local-runtime floor |
| `HP-A1-*` | first-floor evidence | `python3 scripts/practical_alpha1_attach.py check-all --format json` | attach accept/reject/deferred rows |
| `TR-A1-*` | first-floor evidence | `python3 scripts/practical_alpha1_transport.py check-all --format json` | local TCP / Docker Compose TCP evidence |
| `VIS-A1-*` | first-floor evidence | `python3 scripts/practical_alpha1_export_devtools.py check-all --format json` | observer-safe export panels |
| `SL-A1-*` | first-floor evidence | `python3 scripts/practical_alpha1_save_load.py check-all --format json` | local-only save/load evidence |
| `AV-A1-*` | first-floor evidence | `python3 scripts/practical_alpha1_avatar.py check-all --format json` | placeholder/custom preview and fallback boundary |
| `PE2E-*` | first-floor evidence | `python3 scripts/practical_alpha1_product_preview.py check-all --format json` | exact-evidence preview bundles; not same-session runtime |
| `PA1W-*` | bounded workflow evidence | `python3 scripts/practical_alpha1_integrated_workflow.py check-all --format json` | bounded developer workflow; not product/public-ready α-1 |

## Alpha-0 Evidence Reference

| Stage | Classification | Validation anchor | Current reading |
|---|---|---|---|
| A..F | current-scope evidence | stage-specific helper closeouts under `scripts/alpha_*` | evidence references only; not operational workflow completion |

## Required Operational Sample Matrix Status

| Required family | Current closest evidence | Gap |
|---|---|---|
| α-0.5 accepted local dispatch / stale membership reject / save-load | `OA05-*`, `RUN-*`, `SL-A1-*` | none within bounded α-0.5 workflow |
| α-0.8 hot-plug lifecycle | `OA08-*`, `HP-A1-*`, `VIS-A1-*` | distributed detach / ordering later |
| α-0.9 devtools panels | `OA09-*`, `VIS-A1-*` | final viewer/telemetry ABI later |
| product alpha release candidate | `product_alpha1_release_check.py check-all` | broader distribution decision later |
| operational suite | `operational_product_samples.py check-all` | final catalog decision later |
| Mir-owned computation | no current runnable anchor | `P-COMP-01` scaffold is actualized; `P-COMP-02..04` implementation half remains; current `AddOne` is host-boundary evidence only |
| Transform / PoseGraph | no current runnable anchor | `P-POSE-01` scaffold is actualized; `P-POSE-02` implementation half remains; no-split-frame is still docs/spec boundary only |
| projection / engine adapter boundary | no current runnable anchor beyond inventory scaffolds | `P-PROJ-01` / `P-ENG-01` scaffolds are actualized; no codegen or engine execution |

## Validation Anchors For Current Audit

```bash
python3 -m unittest scripts.tests.test_mir_computational_samples
python3 -m unittest scripts.tests.test_posegraph_samples
python3 -m unittest scripts.tests.test_projection_boundary_samples
python3 -m unittest scripts.tests.test_engine_adapter_boundary_samples
python3 scripts/mir_computational_samples.py check-all --format json
python3 scripts/posegraph_samples.py check-all --format json
python3 scripts/projection_boundary_samples.py check-all --format json
python3 scripts/engine_adapter_boundary_samples.py check-all --format json
python3 -m unittest scripts.tests.test_validate_docs scripts.tests.test_product_alpha1_installed_binary_check scripts.tests.test_product_alpha1_release_check scripts.tests.test_operational_product_samples
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release
python3 scripts/product_alpha1_installed_binary_check.py --format json check-all --out /tmp/mirrorea-alpha1-installed-binary-check
python3 scripts/operational_product_samples.py check-all --format json
```

## Recent Validation Log

| Timestamp | Scope | Status | Notes |
|---|---|---|---|
| 2026-05-21 19:56 JST | front-half scaffold sync | pass | projection / engine scaffolds, validators, snapshot docs, and helper validation anchors were synchronized; next reopen point is `P-COMP-02` |
| 2026-05-21 19:44 JST | `P-ENG-01` engine adapter scaffold actualization | pass | `samples/product-alpha1/engine-adapter/`, `matrix.json`, helper, unit test, and `plan/56` were actualized; `run wasm-sandbox` rejects as `planned_only` |
| 2026-05-21 19:40 JST | `P-PROJ-01` projection scaffold actualization | pass | `samples/product-alpha1/projection/`, `matrix.json`, helper, unit test, and `plan/55` were actualized; `run proj-01-server-client-target-manifest` rejects as `planned_only` |
| 2026-05-21 19:41 JST | `P-POSE-01` PoseGraph scaffold actualization | pass | `samples/product-alpha1/posegraph/`, `matrix.json`, helper, unit test, validator registration, and snapshot docs were synchronized; `run pose-04-no-split-frame-positive` and `run pose-05-split-frame-negative` reject as `planned_only` |
| 2026-05-21 19:22 JST | `P-COMP-01` computational scaffold actualization | pass | `samples/product-alpha1/computational/`, `matrix.json`, helper, unit test, validator registration, and snapshot docs were synchronized; `run comp-02-pure-add-one` rejects as `planned_only` |
| 2026-05-21 18:56 JST | `P-COMP-00B` autonomous execution contract | docs/spec planned | integrated reviewer findings: front-half closeout before implementation, `mir-semantics` computational module target, projection/provider compatibility, provider rollback/replay/cut policy; no new runnable sample roots or helpers |
| 2026-05-21 17:35 JST | `P-COMP-00` Mir computational core rebaseline | docs/spec planned | added boundary-fixed rows for computational core, PoseGraph, projection/backend, and engine adapter; no new runnable sample roots or helpers |
| 2026-05-07 13:08 JST | `P-OPS-27` alpha-1 usability and snapshot-doc audit | pass | product release check, installed-binary probe, and operational suite check-all were rerun with Docker included; overview docs were compacted; `mir_hilight.html` active sample inventory was resynced |
| 2026-05-07 12:25 JST | `P-OPS-26` later user-final distribution decision scoping | pass | `user_final_decision_scope` fixed current delivery unit, current catalog scope, and user-spec-required next gate |
| 2026-05-07 10:22-12:03 JST | `P-OPS-20..25` queue and scope hardening | pass | distribution, room-chat, portal/shard starter, Sugoroku, and widening-queue scope blocks were added or narrowed |
| 2026-05-06 21:12-2026-05-07 09:57 JST | `P-OPS-01..19` operational suite and adoption probe | pass | operational suite roots, starter docs, backend inventory, installed-binary probe, and shipped surface were actualized or narrowed |
| 2026-05-05 | `P-A1-25..31` product alpha-1 release-candidate line | pass | product alpha boundary, CLI/schema, runtime, save/load, transport/devtools, native bundle, and release check were actualized |
