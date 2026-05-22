# samples_progress

Last updated: 2026-05-22 15:55 JST

Current repo-local focus: current-L2 runnable floor, practical alpha-1 evidence, bounded operational α-0.5 / α-0.8 / α-0.9 workflows, product alpha-1 release candidate, installed-binary adoption probe, canonical operational product sample suite, `P-PAT-01` minimal alpha-1 pattern verification, and the first `P-MIR-01..04` plus `P-POSE-03..04` plus `P-PROJ-02..04` plus `P-ENG-02` Full System V1 parser+checker+bounded-effectful-runtime+PoseGraph-runtime+pose-save/devtools+projection-schema+local-role-split+provider-admission lane with `P-ENG-03` promoted. `samples/alpha/` remains alpha-0 evidence; `samples/practical-alpha1/` remains first-floor fixture evidence; `samples/product-alpha1/` is the current product alpha root. `samples/full-system-v1/computational/`、`samples/full-system-v1/avatar-pose/`、`samples/full-system-v1/projection/`、`samples/full-system-v1/server-client/`、`samples/full-system-v1/provider-adapter/` は evidence-closed だが、wider Full System V1 roots remain non-workflow-ready until later packages add renderer pose backend and broader source-first families. Docker skip paths are partial local probes, not release-candidate evidence.

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
| Mir computational core | bounded first-floor plus host-boundary evidence | `python3 scripts/mir_computational_samples.py check-all --format json` | `specs/28` / `plan/53`, `samples/product-alpha1/computational/add-one-pure-mir/package.mir.json`, `samples/product-alpha1/computational/host-io-internal-transform/`, helper, unit tests, and runtime tests now prove one direct `ReadInt -> add_one -> WriteInt` row, 5 accepted helper rows, 5 expected runtime rejection rows, 1 direct accepted host read/write boundary row, and 3 expected `check` rejections | broader computational publish / observe / witness / handoff widening remains later |
| Transform / PoseGraph | evidence-closed helper no-split-frame line | `python3 scripts/posegraph_samples.py check-all --format json` | `specs/29` / `plan/54`, `samples/product-alpha1/posegraph/`, `matrix.json`, helper-only `package.mir.json`, and unit test now classify 7 planned rows, 1 accepted row, and 1 `violation_export` row while keeping same-client same-observation-snapshot explicit | pose-aware save/load, devtools panel family, and anchor-switch/reacquire rows remain later |
| Full System V1 PoseGraph runtime | evidence-closed bounded runtime lane | `python3 scripts/posegraph_runtime_samples.py check-all --format json` | `specs/37` / `plan/61`, `samples/full-system-v1/avatar-pose/`, `crates/mir-runtime::posegraph_runtime`, `cargo test -p mir-runtime --test posegraph_runtime -- --nocapture`, and unit test now prove 5 accepted rows, 1 `violation_export`, and 3 `runtime_rejection` rows with no-split-frame, anchor-switch frontier, stale-anchor membership rejection, fallback-only reacquire requirement, bounded save/load admissibility, and observer-safe PoseGraph/devtools export | projection preservation, renderer pose backend, and distributed durable pose save/load remain later |
| projection/backend boundary | boundary-fixed, planned scaffold actualized | `python3 scripts/projection_boundary_samples.py check-all --format json` | `specs/30` / `plan/55`, `samples/product-alpha1/projection/`, `matrix.json`, and helper/unit test now classify planned rows and reject `run` as `planned_only` | no projection codegen, server/client binary split, or backend execution yet |
| Full System V1 projection IR | evidence-closed bounded projection lane | `python3 scripts/projection_v1_samples.py check-all --format json` | `specs/36` / `plan/62`, `samples/full-system-v1/projection/`, `crates/mir-semantics::full_system_v1::projection`, `crates/mir-runtime::full_system_v1_projection`, `cargo test -p mir-runtime --test projection_ir -- --nocapture`, `cargo test -p mirrorea-cli --test full_system_v1_cli -- --nocapture`, and the `mirrorea-alpha project-full-v1` surface now prove 1 accepted row and 3 rejection rows with source-derived target manifests, packet schemas, FFI schemas, source-owned capability/failure rows, preservation reports, generated projection-artifact/rejection-report bundles, unassigned-place rejection, save/load ownership rejection, explicit client-write authority rejection, payload-shape mismatch rejection, and same-shape heterogeneous effect-contract rejection | renderer pose backend and final transport semantics remain later |
| Full System V1 local role split | evidence-closed bounded role-run lane | `python3 scripts/projection_v1_samples.py check-all --format json` | `specs/36` / `plan/62`, `samples/full-system-v1/server-client/`, `crates/mir-runtime::full_system_v1_local_split`, `cargo test -p mir-runtime --test projection_ir -- --nocapture`, `cargo test -p mirrorea-cli --test full_system_v1_cli -- --nocapture`, and `mirrorea-alpha run-full-v1-split` now prove 1 accepted same-binary server/client role-run row plus 1 undeclared-entry rejection row with generated local-split inventory reports | renderer pose backend, Docker/deployment planner, and final split artifacts remain later |
| Full System V1 provider admission | evidence-closed bounded provider lane | `python3 scripts/provider_admission_samples.py check-all --format json` | `specs/38` / `plan/63`, `samples/full-system-v1/provider-adapter/`, `crates/mir-runtime::full_system_v1_provider_admission`, `cargo test -p mir-runtime --test provider_admission -- --nocapture`, `cargo test -p mirrorea-cli --test full_system_v1_cli -- --nocapture`, and `mirrorea-alpha admit-provider-v1` now prove viewer-diagnostic inventory admission, WASM inventory-only admission, over-capability rejection, missing rollback policy rejection, native-disabled rejection, and generated provider-admission reports while preserving packet/FFI schema refs, capability/authority/redaction/retention checks, and disabled-native default | renderer pose backend, arbitrary native/WASM execution, and final provider ABI remain later |
| engine/WASM/FFI adapter boundary | boundary-fixed, planned scaffold actualized | `python3 scripts/engine_adapter_boundary_samples.py check-all --format json` | `specs/31` / `plan/56`, `samples/product-alpha1/engine-adapter/`, `matrix.json`, and helper/unit test now classify planned provider rows and reject `run` as `planned_only` | no engine integration, final FFI ABI, or admitted native/WASM execution yet |
| minimal alpha-1 pattern verifier | evidence-closed maintenance helper | `python3 scripts/minimal_alpha1_patterns.py check-all --format json` | compact strict verifier over exact computational / PoseGraph / projection / engine-adapter counts, expected rejection IDs, compatibility rows, and inventory execution policies | not a new runtime semantics layer; heavy workflow anchors optional |
| autonomous execution contract | boundary-fixed, no sample claim | docs validation plus package helpers | `specs/32` / `plan/57` define front-half closeout, implementation half, package cadence, and close protocol; the current chain has passed all-up closeout validation | reopen only when a new package line is promoted |
| Full System V1 roadmap | evidence-closed parser+checker+bounded-effectful-runtime+PoseGraph-runtime+pose-save/devtools+projection-schema+role-split+provider-admission floor, wider line planned | `python3 scripts/textual_mir_samples.py check-all --format json`, `python3 scripts/full_system_v1_samples.py check-all --format json`, `python3 scripts/posegraph_runtime_samples.py check-all --format json`, `python3 scripts/projection_v1_samples.py check-all --format json`, `python3 scripts/provider_admission_samples.py check-all --format json`, `cargo test -p mir-runtime --test full_system_v1_session -- --nocapture`, `cargo test -p mir-runtime --test posegraph_runtime -- --nocapture`, `cargo test -p mir-runtime --test projection_ir -- --nocapture`, and `cargo test -p mir-runtime --test provider_admission -- --nocapture` | `specs/33..38`, `plan/58..63`, `samples/full-system-v1/computational/`, `samples/full-system-v1/avatar-pose/`, `samples/full-system-v1/projection/`, `samples/full-system-v1/server-client/`, `samples/full-system-v1/provider-adapter/`, `scripts/textual_mir_samples.py`, `scripts/full_system_v1_samples.py`, `scripts/posegraph_runtime_samples.py`, `scripts/projection_v1_samples.py`, `scripts/provider_admission_samples.py`, `crates/mir-ast::textual_alpha`, `crates/mir-semantics::full_system_v1`, `crates/mir-runtime::full_system_v1_session`, `crates/mir-runtime::posegraph_runtime`, `crates/mir-runtime::full_system_v1_projection`, `crates/mir-runtime::full_system_v1_local_split`, and `crates/mir-runtime::full_system_v1_provider_admission` now actualize the first source-first parser lane, typed checker lane, bounded effectful runtime lane, bounded PoseGraph runtime lane, bounded pose save/devtools lane, bounded projection IR plus boundary-schema lane, bounded local role-split lane, and bounded provider-admission lane | renderer pose backend, broader source-first operational families, and release check remain later |

## Product Alpha Root Status

| Root | Role | Runnable anchor | Current reading |
|---|---|---|---|
| `samples/product-alpha1/demo/` | release-candidate product demo | `python3 scripts/product_alpha1_release_check.py --format json check-all --out <dir>` | workflow-ready alpha demo; not final product |
| `samples/product-alpha1/operational/` | canonical operational sample suite | `python3 scripts/operational_product_samples.py check-all --format json` | workflow-ready operational suite; not final product |
| `samples/product-alpha1/operational/templates/` | template-only authoring starters | `cargo run -q -p mirrorea-cli -- check <template> --format json` | `world-core`, `membership-chat`, `sugoroku-world` starters only |
| `samples/product-alpha1/operational/future/` | future boundary inventory | JSON validation / docs references | retained blueprint/profile inventory; non-executable unless paired with active roots |
| `samples/product-alpha1/computational/` | two direct runtime/check surfaces plus ten helper-executable first-floor rows | `python3 scripts/mir_computational_samples.py check-all --format json` | `add-one-pure-mir/` is executable via `package.mir.json`; `comp-03` rows execute through helper package contracts; `comp-04` now adds one direct accepted host read/write row plus three expected `check` rejections |
| `samples/product-alpha1/posegraph/` | bounded Transform / PoseGraph helper evidence | `python3 scripts/posegraph_samples.py check-all --format json` | helper-only `package.mir.json` plus representative `.mir` files and `matrix.json` exist; `pose-04` is accepted, `pose-05` is `violation_export`, and 7 rows remain planned |
| `samples/product-alpha1/projection/` | planned projection boundary roots | `python3 scripts/projection_boundary_samples.py check-all --format json` | planned-only scaffold actualized; representative inventory JSON files and `matrix.json` exist, but no codegen/runtime row yet |
| `samples/product-alpha1/engine-adapter/` | planned engine/provider boundary roots | `python3 scripts/engine_adapter_boundary_samples.py check-all --format json` | planned-only scaffold actualized; representative contract JSON files and `matrix.json` exist, but no admitted provider row yet |

## Full System V1 Planned Sample Line

| Planned root | Intended role | Current reading |
|---|---|---|
| `samples/full-system-v1/computational/` | textual Mir computational source samples | evidence-closed parser+checker+bounded-effectful-runtime floor; parser helper validates 2 positive rows and 8 negative rows, typed checker helper validates 3 positive rows and 9 negative rows with explicit obligations, imported-module semantic closure, and ambiguous import rejection, and runtime helper validates 8 positive rows and 9 negative rows with compute traces, effect-session summaries, and static/runtime rejection split |
| `samples/full-system-v1/world-core/` | source-first WorldCore operational root | planned only; current executable root remains `samples/product-alpha1/operational/world-core/` |
| `samples/full-system-v1/membership-chat/` | source-first MembershipChat operational root | planned only; current executable root remains `samples/product-alpha1/operational/membership-chat/` |
| `samples/full-system-v1/sugoroku-world/` | source-first SugorokuWorld operational root | planned only; current executable root remains `samples/product-alpha1/operational/sugoroku-world/` |
| `samples/full-system-v1/avatar-pose/` | runtime PoseGraph source sample family | evidence-closed bounded runtime root; `matrix.json` now carries 9 executable rows with save/load/devtools export, and `python3 scripts/posegraph_runtime_samples.py check-all --format json` is the anchor |
| `samples/full-system-v1/projection/` | `FS-06` projection IR and boundary-schema evidence | evidence-closed bounded projection root; `matrix.json` now carries 4 executable rows, generated projection-artifact/rejection-report bundles, and `python3 scripts/projection_v1_samples.py check-all --format json` is the anchor |
| `samples/full-system-v1/server-client/` | `FS-07` local server/client role-run evidence | evidence-closed bounded role-run root; `matrix.json` now carries 2 executable rows with 1 accepted same-binary role-run row, 1 undeclared-entry rejection row, generated local-split inventory reports, and `python3 scripts/projection_v1_samples.py check-all --format json` is the anchor |
| `samples/full-system-v1/provider-adapter/` | provider admission evidence | evidence-closed bounded provider-admission root with 5 executable rows, generated `provider-admission-report.json`, and `python3 scripts/provider_admission_samples.py check-all --format json` as the anchor |

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
| Mir-owned computation | bounded first-floor evidence plus direct host-boundary closure | `P-COMP-02` proves one runnable Mir-owned row, `P-COMP-03` proves positive / negative first-floor rows, and `P-COMP-04` proves one accepted host read/write boundary row plus three expected declaration rejects; current legacy `AddOne` is still host-boundary evidence only |
| Transform / PoseGraph | helper-backed anchor plus source-first runtime/save-load/devtools anchor exist | `P-POSE-02` actualizes one same-snapshot accepted row and one split-snapshot `violation_export` row, `P-POSE-03` actualizes 4 accepted / 1 violation / 3 runtime rejection source-first rows under `samples/full-system-v1/avatar-pose/`, and `P-POSE-04` widens that root to 5 accepted / 1 violation / 3 runtime rejection rows with bounded save/load admissibility and observer-safe devtools export; distributed durable pose save/load remains later |
| projection / engine adapter boundary | bounded projection IR + role-run + provider-admission anchors exist; product-alpha engine/provider remains inventory-only | `P-PROJ-01` / `P-ENG-01` scaffolds remain as comparison anchors, while `P-PROJ-03` actualizes `samples/full-system-v1/projection/`, `P-PROJ-04` actualizes `samples/full-system-v1/server-client/`, and `P-ENG-02` actualizes `samples/full-system-v1/provider-adapter/`; renderer pose backend and arbitrary native/WASM execution remain later |

## Validation Anchors For Current Audit

```bash
python3 -m unittest scripts.tests.test_mir_computational_samples
python3 -m unittest scripts.tests.test_posegraph_samples
python3 -m unittest scripts.tests.test_projection_boundary_samples
python3 -m unittest scripts.tests.test_engine_adapter_boundary_samples
python3 -m unittest scripts.tests.test_minimal_alpha1_patterns
python3 -m unittest scripts.tests.test_textual_mir_samples
python3 -m unittest scripts.tests.test_full_system_v1_samples
python3 -m unittest scripts.tests.test_posegraph_runtime_samples
python3 -m unittest scripts.tests.test_projection_v1_samples
python3 -m unittest scripts.tests.test_provider_admission_samples
cargo test -p mir-ast --test textual_mir_alpha -- --nocapture
cargo test -p mir-semantics --test typed_ir_interpreter -- --nocapture
cargo test -p mir-runtime --test full_system_v1_session -- --nocapture
cargo test -p mir-runtime --test posegraph_runtime -- --nocapture
cargo test -p mir-runtime --test projection_ir -- --nocapture
cargo test -p mir-runtime --test provider_admission -- --nocapture
cargo test -p mirrorea-cli --test full_system_v1_cli -- --nocapture
python3 scripts/textual_mir_samples.py matrix --format json
python3 scripts/textual_mir_samples.py check-all --format json
python3 scripts/full_system_v1_samples.py matrix --format json
python3 scripts/full_system_v1_samples.py check-all --format json
python3 scripts/posegraph_runtime_samples.py matrix --format json
python3 scripts/posegraph_runtime_samples.py check-all --format json
python3 scripts/projection_v1_samples.py matrix --format json
python3 scripts/projection_v1_samples.py check-all --format json
python3 scripts/provider_admission_samples.py matrix --format json
python3 scripts/provider_admission_samples.py check-all --format json
python3 scripts/mir_computational_samples.py check-all --format json
python3 scripts/posegraph_samples.py check-all --format json
python3 scripts/projection_boundary_samples.py check-all --format json
python3 scripts/engine_adapter_boundary_samples.py check-all --format json
python3 scripts/minimal_alpha1_patterns.py check-all --format json
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
| 2026-05-22 15:55 JST | `P-ENG-02` provider admission closeout | pass | `crates/mir-runtime::full_system_v1_provider_admission`、`samples/full-system-v1/provider-adapter/`、`scripts/provider_admission_samples.py`、runtime/CLI tests、`mirrorea-alpha admit-provider-v1` が同期し、viewer-diagnostic inventory accepted row、WASM inventory-only accepted row、over-capability rejection、missing rollback policy rejection、native-disabled rejectionを固定したうえで current promoted package を `P-ENG-03` に更新した |
| 2026-05-22 15:12 JST | `P-PROJ-04` local server/client split closeout | pass | `crates/mir-runtime::full_system_v1_local_split`、`samples/full-system-v1/server-client/`、`scripts/projection_v1_samples.py`、runtime/CLI tests、`mirrorea-alpha run-full-v1-split` が同期し、1 accepted role-run row と 1 undeclared-entry rejection row を固定したうえで current promoted package を `P-ENG-02` に更新した |
| 2026-05-22 14:46 JST | `P-PROJ-03` boundary schemas closeout | pass | `crates/mir-semantics::full_system_v1::projection` and `crates/mir-runtime::full_system_v1_projection` now preserve packet/FFI schema bundles, payload-shape mismatch rejection, same-shape heterogeneous effect-contract rejection, projection-artifacts/rejection-report outputs, and 4 executable `samples/full-system-v1/projection/` rows; current promoted package is `P-PROJ-04` |
| 2026-05-22 14:10 JST | `P-PROJ-02` projection IR closeout | pass | `crates/mir-semantics::full_system_v1::projection` and `crates/mir-runtime::full_system_v1_projection` now lower accepted source plus `projection.request.json` into source-derived target manifests and preservation reports with source-owned capability/failure rows, unassigned-place rejection, save/load ownership rejection, and 1 client-write rejection row; current promoted package is `P-PROJ-03` |
| 2026-05-22 13:13 JST | `P-POSE-04` pose save/devtools closeout | pass | `crates/mir-runtime::posegraph_runtime` now rejects accepted-path save/load mismatches as `save_load_inadmissible`, exports observer-safe PoseGraph/devtools panels, synchronizes 9 executable `avatar-pose` rows, and promotes the next package to `P-PROJ-02` |
| 2026-05-22 12:56 JST | `P-POSE-03` runtime PoseGraph closeout | pass | reviewer-found anchor-switch ordering/frontier coherence gap, missing-witness reacquire gap, switch-membership stale gap, and helper closeout drift were fixed; `samples/full-system-v1/avatar-pose/`, runtime test, helper script, and expected rows are synchronized, and current promoted package is `P-POSE-04` |
| 2026-05-22 12:19 JST | `P-MIR-04` reviewer-fix closeout | pass | bind contract post-bind scope, pure/negative runtime empty effect-session summaries, and host-output/non-transport quiescence separation were resynchronized; current promoted package remains `P-POSE-03` |
| 2026-05-22 11:58 JST | `P-MIR-04` effectful integration closeout | pass | `samples/full-system-v1/computational/` widened to bounded transition/effect rows for host boundary, publish/observe, witness/handoff, and local atomic-cut negatives; current promoted package is `P-POSE-03` |
| 2026-05-22 11:37 JST | `P-MIR-03` computational interpreter closeout | pass | docs snapshot, report, and major anchors were synchronized; `samples/full-system-v1/computational/` remains evidence-closed and the current promoted package is `P-MIR-04` |
| 2026-05-22 11:30 JST | `P-MIR-03` computational interpreter | pass | `crates/mir-semantics::full_system_v1`, `crates/mir-runtime::full_system_v1_session`, `scripts/full_system_v1_samples.py`, and `samples/full-system-v1/computational/runtime-matrix.json` 6-positive/4-negative source-derived runtime rows with compute trace, observer-safe summary, and static/runtime rejection split were synchronized; next promoted package is `P-MIR-04` |
| 2026-05-22 10:59 JST | `P-MIR-02` typed IR and checker | pass | `crates/mir-semantics::full_system_v1`, `typed_ir_interpreter` tests, `scripts/full_system_v1_samples.py`, and `samples/full-system-v1/computational/typed-ir-matrix.json` 3-positive/9-negative rows plus imported-module semantic closure and ambiguous import rejection were synchronized; validator/report heading drift was also closed and the next promoted package is `P-MIR-03` |
| 2026-05-22 10:01 JST | `P-MIR-01` textual Mir alpha grammar | pass | `crates/mir-ast::textual_alpha`, path-aware unresolved import diagnostics, expression spans, `scripts/textual_mir_samples.py`, and `samples/full-system-v1/computational/` 2-positive/8-negative rows were synchronized; next promoted package is `P-MIR-02` |
| 2026-05-22 03:21 JST | `P-FS-00` Full System V1 roadmap rebaseline | docs/spec planned | source-first Full System V1 roadmap added; `samples/full-system-v1/` remains planned only and the next implementation package is `P-MIR-01` |
| 2026-05-21 23:37 JST | `P-PAT-01` minimal alpha-1 pattern verifier | pass | `scripts/minimal_alpha1_patterns.py`, unit test, hands-on / research summary, and snapshot docs were added; strict default check covers computational, PoseGraph, projection, and engine-adapter rows |
| 2026-05-21 22:11 JST | all-up closeout audit | pass | focused helper suites, Cargo regressions, product alpha release check, installed-binary probe, operational suite, docs validator, and source hierarchy checks were rerun; the current self-driven chain is now closed through its planned audit package |
| 2026-05-21 21:57 JST | `P-POSE-02` no-split-frame helper evidence | pass | `samples/product-alpha1/posegraph/`, helper-only `package.mir.json` inputs, matrix, unit test, and snapshot docs were synchronized; `check-all` now reports 1 accepted row, 1 violation row, and 7 planned rows; next reopen point is the all-up closeout audit |
| 2026-05-21 21:27 JST | `P-COMP-04` computational effect boundary | pass | `host-io-internal-transform/` direct accepted/check-rejection rows, helper matrix, product-alpha schema/runtime tests, and snapshot docs were synchronized; `check-all` now reports 7 accepted rows, 5 expected runtime rejections, and 3 expected check rejections; next reopen point is `P-POSE-02` |
| 2026-05-21 20:53 JST | `P-COMP-03` computational first-floor widening | pass | `mir-semantics` computational core, product-alpha schema/runtime tests, helper matrix, and sample rows were synchronized; `check-all` now reports 6 accepted rows, 5 expected runtime rejections, and `P-COMP-04` as the next reopen point |
| 2026-05-21 20:27 JST | `P-COMP-02` Mir-owned add-one | pass | `mir-semantics` computational core, product-alpha computational package/runtime lane, helper, and docs were synchronized; `run comp-02-pure-add-one` is accepted and the next reopen point is `P-COMP-03` |
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
