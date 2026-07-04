# scripts

この directory は、**active runner、repo-local helper、detached/export assist、storage/env、tests** を置く。

## current taxonomy

### front-door checks and active runners

- `check_source_hierarchy.py`
  required root docs / canon entry docs / specs / plan / support directory が存在するかを見る structural check。current line では `CANON.md`、`mirrorea_canon/` entry files、`specs/13..43`、current numbered `plan/00..119` と source-traceability / maintenance rules、Surface Mir guides、Surface parser / indexed-state / elaboration / role-admission / source-patch / devtools / operational-source scripts/tests、Full System V1 / Surface active sample roots、`samples/alpha/`、Product Alpha demo entry files、`docs/hands_on/`、`docs/research_abstract/`、`sub-agent-pro/alpha-0/`、`sub-agent-pro/alpha-1/`、`sub-agent-pro/product-alpha1-001/`、`sub-agent-pro/operational-product-sample-001/`、`sub-agent-pro/full-system-completion-001/` も structural presence の対象に入る。文書内容、stale wording、normative consistency、report template completeness は判定しない。
- `validate_docs.py`
  required documentation scaffold、root canon notices、reader-facing source-hierarchy wording lint、active reader-facing host-specific repo path lint、numbered `plan/*.md` registration guard、`progress.md` / `tasks.md` top `最終更新` freshness、required snapshot heading order、numbered report、report template closeout headings、latest numbered report の required heading presence / order、empty required section、unresolved update-status placeholder を確認する docs validation check。current line では snapshot docs、`CANON.md`、`mirrorea_canon/` entry files、`samples/README.md` / `scripts/README.md`、`.docs/`、`docs/hands_on/`、`docs/research_abstract/`、`samples/alpha/README.md`、`samples/product-alpha1/README.md`、`samples/product-alpha1/demo/README.md`、`samples/product-alpha1/demo/package.mir.json`、active sample roots、Surface Mir guides、Surface parser / indexed-state / elaboration / role-admission / source-patch / devtools / operational-source scripts/tests、`samples/full-system-v1-surface/{syntax,indexed-state,elaboration,role-admission,source-patch,devtools,world-core,membership-chat,sugoroku-world,portal-worldlink,two-shard-hard-boundary,gradient-observation}/`、current numbered `plan/00..119` と source-traceability / maintenance rules、`specs/13..43` も required scaffold または lint 対象に入る。historical report 全体の semantic validation、sample execution、Cargo validation は別 command の責務。
- `clean_near_end_samples.py`
- `current_l2_guided_samples.py`
  compatibility wrapper for `list` / `smoke-all` / `closeout` over `clean_near_end_samples.py`
- `sugoroku_world_samples.py`
- `avatar_follow_samples.py`
- `typed_external_boundary_samples.py`
- `network_transport_samples.py`
  runnable helper-local transport canaries are `NET-02` / `NET-03` / `NET-04` / `NET-05`; `NET-01` remains a reported Sugoroku loopback parity anchor rather than a standalone sample ID
- `projection_codegen_samples.py`
- `visual_debugger_viewer_samples.py`
- practical alpha-1 initial front-door is currently cargo-based rather than script-based
  - `cargo test -p mir-ast practical_alpha1_front_door -- --nocapture`
  - this exercises `samples/practical-alpha1/` through `crates/mir-ast::practical_alpha1`
- practical alpha-1 first checker floor now has an alpha-local script surface
  - `python3 scripts/practical_alpha1_check.py check-all --format json`
  - this exercises `samples/practical-alpha1/packages/chk-*/` through `crates/mir-ast::practical_alpha1_checker`
  - it is a non-final checker-only command and does not emit runtime plans
- practical alpha-1 first local-runtime floor now has an alpha-local script surface
  - `python3 scripts/practical_alpha1_run_local.py check-all --format json`
  - this exercises `samples/practical-alpha1/packages/run-*/` through `crates/mir-ast::practical_alpha1_runtime_plan` and `crates/mir-runtime::practical_alpha1_local_runtime`
  - it consumes checked practical package input through a distinct runtime-plan boundary
  - current actualized rows are `RUN-01..04`
  - it is a non-final first-floor local-runtime command and does not claim same-session operational α-0.5 runtime, Docker transport, package/hot-plug, save/load, or final public runtime/devtools API
  - practical `run-docker` remains later work
- practical alpha-0.5 session carrier now has an alpha-local script surface
  - `python3 scripts/practical_alpha05_session.py check-all --format json`
  - this exercises `samples/practical-alpha1/packages/run-*/` and `packages/oa05-07-add-one-host-io/` through `crates/mir-runtime::practical_alpha05_session`, `crates/mir-runtime::practical_alpha05_host_io`, and exact `SL-A1-02` / `VIS-A1-05` source evidence
  - current actualized rows are `OA05-01..07`
  - it actualizes same-session `start` / `observe` / `save` / `load` over the bounded local runtime carrier plus session-bound event DAG / observer-safe export and one minimal typed external `AddOne` host-I/O adapter lane
  - this lane is not Mir-owned computational-core evidence; pure `add_one` in Mir is tracked under `specs/28` / `plan/53`
  - it does not claim same-session hot-plug runtime, distributed durable save/load, or final public runtime/devtools API
- practical alpha-0.8 same-session hot-plug lane now has an alpha-local script surface
  - `python3 scripts/practical_alpha08_session_hotplug.py check-all --format json`
  - this exercises exact `HP-A1-01..07` / `AV-A1-03` package evidence through `crates/mir-runtime::practical_alpha08_hotplug_session` and the live `crates/mir-runtime::practical_alpha05_session` carrier
  - current actualized rows are `OA08-01..10`
  - it actualizes same-session `attach` / `observe` over the bounded session carrier plus accepted/rejected/deferred / activation cut / object preview / fallback companion visibility summaries
  - rejected attach attempts remain non-mutating for active runtime state but are now preserved as session-carried observation entries for α-0.9 export
  - it does not claim accepted detach execution, distributed durable save/load, or final public runtime/devtools/hot-plug API
- practical alpha-0.9 session-bound devtools lane now has an alpha-local script surface
  - `python3 scripts/practical_alpha09_devtools.py check-all --format json`
  - this exercises one enriched `practical_alpha05_session` carrier through typed host-I/O, same-session hot-plug, local save/load, and `crates/mir-runtime::practical_alpha09_devtools`
  - current actualized rows are `OA09-01..09`
  - it exports event DAG, local route trace, membership timeline, witness relation, hot-plug lifecycle, fallback degradation, save-load timeline, observer-safe redacted view, and retention/on-demand trace from the same session carrier
  - `render-html` emits a non-final static HTML viewer over the same session-bound payload
  - it does not claim final public viewer/telemetry ABI, durable audit, remote retained-artifact retrieval, WAN/federation route trace, distributed durable save/load, or product-ready alpha-1
- practical alpha-1 integrated workflow now has an alpha-local script surface
  - `python3 scripts/practical_alpha1_integrated_workflow.py check-all --format json`
  - this composes existing first-floor exact evidence and bounded operational carriers through `PA1W-01..08`
  - it covers source front-door, checker, same-session runtime, typed host-I/O, same-session hot-plug, local save/load, session-bound devtools, product-preview evidence, negative guards, and explicit non-final stop lines
  - it also revalidates exact devtools/product-preview evidence through `VIS-A1-01` and `PE2E-01/02/07`
  - it does not claim final public parser/checker/runtime API, final public viewer/telemetry ABI, distributed durable save/load, WAN/federation, native avatar execution, or product-ready alpha-1
- product alpha-1 CLI/schema first cut now has a Rust CLI surface
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
  - `cargo run -q -p mirrorea-cli -- demo --out /tmp/mirrorea-alpha1-demo --format json`
  - `python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release`
  - `python3 scripts/product_alpha1_installed_binary_check.py --format json check-all --out /tmp/mirrorea-alpha1-installed-binary-check`
  - this exercises `samples/product-alpha1/demo/package.mir.json` through `crates/mir-ast::product_alpha1`
  - `check` is implemented for schema acceptance / explicit residual obligations
  - `run-local`、`session`、`attach`、`save`、`load`、`quiescent-save` are implemented for a local file-backed product session carrier through `crates/mir-runtime::product_alpha1_session`
  - `transport` is implemented through `crates/mir-runtime::product_alpha1_transport`; local mode uses loopback TCP and Docker mode uses `samples/product-alpha1/docker/docker-compose.product-alpha1.yml` when Docker / Docker Compose are available
  - `export-devtools` / `view` are implemented through `crates/mir-runtime::product_alpha1_devtools` as a non-final static HTML / JSON viewer bundle
  - `build-native-bundle` emits a native host launch bundle with compiled CLI, package bundle, observer-safe devtools assets, manifest, launch metadata, run script, verification report, and provenance metadata
  - `demo` runs the release-candidate workflow and writes reports, concrete non-final devtools assets, an observer-safe session artifact, an admin/debug session store, and native-bundle artifacts under the chosen output directory
  - `demo --skip-docker` is a partial local probe and does not claim release-candidate readiness
  - `product_alpha1_release_check.py check-all` runs the validation floor, focused tests, command family, native bundle run script probes, and JSON payload semantics for clean-clone validation
  - `product_alpha1_installed_binary_check.py check-all` builds `target/debug/mirrorea-alpha`, runs the built binary directly for `check` / `build-native-bundle` / `demo`, replays bundle `run.sh check` / `run.sh view`, and reports the current `installed_binary_plus_native_host_launch_bundle` adoption candidate without claiming final public CLI/API/ABI or final packaging
  - the same helper now also reports the current hardening target as machine-readable compatibility scope:
    versioned `package.mir.json`, documented `mirrorea-alpha` command family, native host launch bundle replay surface
  - and it reports the current `shipped_surface` unit separately:
    built-binary `check` / `build-native-bundle` / `demo`, bundle replay `run.sh check` / `run.sh view`, bundled CLI / package root / `manifest.json` / `launch.json` / `run.sh` / `README.md`, observer-safe supporting artifacts, while keeping other bundled reports as evidence-only
  - and it reports a machine-readable `distribution_scope`:
    current delivery unit is developer-built `mirrorea-alpha` plus a locally generated native host launch bundle, while archive / installer / system-package / auto-update / hosted-service shapes remain undefined
  - `operational_product_samples.py` is the orchestration helper for `samples/product-alpha1/operational/`; it keeps `mirrorea-alpha` as the canonical entrypoint and bundles `list`, `check-all`, `run-world-core`, `run-membership-chat`, `run-sugoroku`, `run-portal-worldlink`, `run-two-shard-hard-boundary`, `run-two-shard-gradient-observation`, `attach-layers`, `transport-local`, `transport-docker`, `export-devtools`, `build-native-bundle`, and `release-check`
  - `run-membership-chat`, `run-sugoroku`, `run-portal-worldlink`, `run-two-shard-hard-boundary`, `run-two-shard-gradient-observation`, `export-devtools`, `release-check`, and `check-all` now include bounded operational semantic checks for room-oriented host-I/O, Sugoroku runtime evidence, portal discrete handoff evidence, two-shard hard-boundary evidence, bounded observer-only gradient runtime evidence, and schema-backed projection inventory rather than only command exit status
  - `run-membership-chat`, `release-check`, and `check-all` also report machine-readable `room_chat_scope`, keeping the current lane at bounded single-message room-oriented `ChatText` and leaving multi-message / transport-coupled / room-history / stdio shapes undefined
  - `run-sugoroku` and `check-all` also report machine-readable `sugoroku_scope`, keeping the current carrier at bounded deterministic same-session roll / publish / witness / handoff / stale-membership reject and leaving interactive turn choice / broader negative rows / networked multi-participant control undefined
  - `check-all` also reports machine-readable `portal_shard_starter_scope`, keeping the current authoring boundary at active executable roots with the validated starter catalog intentionally stopping at `templates/sugoroku-world-starter`
  - `check-all` also reports machine-readable `widening_queue_scope`, keeping room-chat reopening, portal/shard starter reopening, and broader Sugoroku reopening non-promoted while advancing the next promoted comparison to `later_user_final_distribution_decision` and marking that comparison as `next_promoted_reopen_requires_user_decision = true`
  - `check-all` also reports machine-readable `user_final_decision_scope`, keeping the current delivery unit on developer-built binary + generated host launch bundle, the current catalog on bounded product alpha-1 narrow showcase, and the broader final distribution / final shared-space catalog line on a user-spec-required gate
  - external developer authoring is intentionally not hidden behind a generic scaffold command; use `docs/hands_on/operational_package_authoring_01.md` with direct `mirrorea-cli check/run-local/session/export-devtools/view` over the validated starter catalog under `samples/product-alpha1/operational/templates/`
  - portal/shard authoring is also intentionally not given a separate starter helper; use `docs/hands_on/operational_portal_shard_starter_boundary_01.md` and the active `portal-worldlink/` / `two-shard-hard-boundary/` roots directly
  - backend feasibility inventory is also docs-first; no generic WASM/LLVM build helper is added beyond the existing `build-native-bundle` host-launch path
  - `future/gradient-observation.profile.json` remains docs-first/profile-first and non-executable, but the bounded `two-shard-gradient-observation/` runtime root and helper command now actualize one observer-only same-session cut without claiming continuous sync or write authority
  - planned Full System V1 source-first naming intentionally uses `gradient-observation/` rather than `two-shard-gradient-observation/`; the shorter root name preserves the same bounded observer-only semantic scope while separating the source-first lane from the Product Alpha operational runtime root
  - it does not claim final product, final public CLI/API, direct `.mir` grammar, WAN/federation, distributed durable save/load R3/R4, final public viewer/telemetry ABI, direct Mir-to-machine-code, signature-is-safety, or arbitrary native execution
- Mir Computational Core / PoseGraph / projection-boundary / engine-adapter helper family is split
  - `python3 scripts/mir_computational_samples.py matrix --format json`
  - `python3 scripts/mir_computational_samples.py check-all --format json`
  - `python3 scripts/mir_computational_samples.py run comp-02-pure-add-one --format json`
  - `python3 scripts/mir_computational_samples.py run comp-03-control-flow-positive --format json`
  - `python3 scripts/mir_computational_samples.py run comp-03-variables-scope-negative --format json`
  - `python3 scripts/mir_computational_samples.py run comp-04-host-io-internal-transform-positive --format json`
  - `python3 scripts/mir_computational_samples.py run comp-04-host-io-internal-transform-negative-undeclared-effect --format json`
  - this helper now executes the `P-COMP-02` bounded computational row under `samples/product-alpha1/computational/add-one-pure-mir/`, executes the `P-COMP-03` first-floor rows through helper package contracts, and executes the `P-COMP-04` direct host read/write boundary row plus three expected `check` rejections
  - it validates matrix/root consistency, checks `host_input_received -> mir_compute_step -> host_output_emitted` for the direct runtime rows, classifies helper rows as `accepted` or `runtime_rejection`, and matches `check_rejection` rows by diagnostic code and detail
  - it does not claim final textual grammar, broad first-floor completion, or backend realization
  - `python3 scripts/posegraph_samples.py matrix --format json`
  - `python3 scripts/posegraph_samples.py check-all --format json`
  - `python3 scripts/posegraph_samples.py run pose-04-no-split-frame-positive --format json`
  - `python3 scripts/posegraph_samples.py run pose-05-split-frame-negative --format json`
  - this helper now actualizes the `P-POSE-02` bounded PoseGraph evidence line under `samples/product-alpha1/posegraph/`
  - it validates matrix/root consistency, executes `pose-04` as accepted same-snapshot evidence, executes `pose-05` as `violation_export`, and keeps the remaining rows `planned_only`
  - it does not claim PoseGraph runtime completion, global simultaneity, or renderer-owned semantics
  - `python3 scripts/projection_boundary_samples.py matrix --format json`
  - `python3 scripts/projection_boundary_samples.py check-all --format json`
  - `python3 scripts/projection_boundary_samples.py run proj-01-server-client-target-manifest --format json`
  - this helper actualizes the `P-PROJ-01` planned-only projection boundary inventory scaffold under `samples/product-alpha1/projection/`
  - it validates matrix/root consistency, exposes accepted/rejected compatibility rows, keeps all rows `planned_only`, and rejects attempted execution as `planned_only`
  - it does not claim server/client code generation, LLVM/backend execution, or final server/client binary split
  - `python3 scripts/engine_adapter_boundary_samples.py matrix --format json`
  - `python3 scripts/engine_adapter_boundary_samples.py check-all --format json`
  - `python3 scripts/engine_adapter_boundary_samples.py run wasm-sandbox --format json`
  - this helper actualizes the `P-ENG-01` planned-only engine / WASM / FFI adapter inventory scaffold under `samples/product-alpha1/engine-adapter/`
  - it validates matrix/root consistency, keeps all provider rows `planned_only`, preserves `NativeExecutionPolicy = Disabled` and `WasmExecutionPolicy = InventoryOnly`, and rejects attempted execution as `planned_only`
  - it does not claim provider admission, arbitrary native/WASM execution, or final engine adapter ABI
- minimal alpha-1 pattern verifier is a compact strict overlay over existing helpers
  - `python3 scripts/minimal_alpha1_patterns.py list --format json`
  - `python3 scripts/minimal_alpha1_patterns.py matrix --format json`
  - `python3 scripts/minimal_alpha1_patterns.py check-all --format json`
  - `python3 scripts/minimal_alpha1_patterns.py run mir-compute-host-io-transform --format json`
  - `python3 scripts/minimal_alpha1_patterns.py run mir-compute-missing-effect-reject --format json`
  - `python3 scripts/minimal_alpha1_patterns.py run posegraph-no-split-frame --format json`
  - `python3 scripts/minimal_alpha1_patterns.py run posegraph-split-frame-violation --format json`
  - default `check-all` fixes exact computational / PoseGraph / projection / engine-adapter counts, expected rejection IDs, compatibility rows, and inventory-only execution policies
  - `check-all --include-workflows --out <dir>` also reruns the heavier product release-candidate and operational suite workflow anchors
  - it does not claim final product, final grammar/API, direct LLVM/native backend, server/client codegen, provider admission, WAN/federation, or distributed durable save-load
- Full System V1 helper family now has parser-floor, typed-checker-floor, bounded runtime commands, bounded source-first operational-suite commands, PoseGraph runtime commands, bounded projection IR + boundary-schema commands, bounded same-binary local role-split commands, bounded provider-admission commands, bounded renderer-pose commands, and a bounded release-check command
  - `python3 scripts/textual_mir_samples.py check-all --format json`
  - `python3 scripts/full_system_v1_samples.py check-all --format json`
  - `python3 scripts/full_system_v1_samples.py operational-matrix --format json`
  - `python3 scripts/full_system_v1_samples.py check-operational-all --format json`
  - `python3 scripts/posegraph_runtime_samples.py check-all --format json`
  - `python3 scripts/projection_v1_samples.py check-all --format json`
  - `python3 scripts/provider_admission_samples.py check-all --format json`
  - `python3 scripts/full_system_v1_release_check.py --format json check-all --out /tmp/mirrorea-full-v1-release`
  - this exercises `samples/full-system-v1/computational/` through `crates/mir-ast::textual_alpha` and the `textual_mir_alpha_parse` example
  - `python3 scripts/full_system_v1_samples.py runtime-matrix --format json` and `run-runtime <sample-id> --format json`
  - the typed-checker helper exercises the same root through `crates/mir-semantics::full_system_v1` and the `full_system_v1_check` example
  - the runtime helper exercises source-derived pure and effectful rows through `crates/mir-runtime::full_system_v1_session` and the `mir_full_system_v1_session` example
  - the same helper now also exercises `samples/full-system-v1/world-core/`, `samples/full-system-v1/membership-chat/`, `samples/full-system-v1/sugoroku-world/`, `samples/full-system-v1/portal-worldlink/`, `samples/full-system-v1/two-shard-hard-boundary/`, and `samples/full-system-v1/gradient-observation/` through generated package-manifest projections plus runtime report expectations via `run-operational <sample-id> --format json`
  - the PoseGraph helper exercises `samples/full-system-v1/avatar-pose/` through `crates/mir-runtime::posegraph_runtime` and the `posegraph_runtime_session` example
  - the projection helper exercises `samples/full-system-v1/projection/` through `crates/mir-semantics::full_system_v1::projection`, `crates/mir-runtime::full_system_v1_projection`, the `mir_full_system_v1_projection` example, the `samples/full-system-v1/server-client/` root through `crates/mir-runtime::full_system_v1_local_split` and the `mir_full_system_v1_local_split` example, and the `mirrorea-alpha project-full-v1` / `run-full-v1-split` CLI surfaces
  - the provider helper exercises `samples/full-system-v1/provider-adapter/` through `crates/mir-runtime::full_system_v1_provider_admission`, the `mir_full_system_v1_provider_admission` example, and the `mirrorea-alpha admit-provider-v1` CLI surface
  - `python3 scripts/renderer_pose_backend_samples.py check-all --format json`
  - the renderer helper exercises `samples/full-system-v1/provider-adapter/renderer-pose-matrix.json` through `crates/mir-runtime::full_system_v1_renderer_pose_backend` and the `mirrorea-alpha render-pose-backend-v1` CLI surface
  - the release-check helper reruns the validation floor, `scripts/tests.test_full_system_v1_release_check`, focused Cargo tests, the bounded source-first helper suite, Product Alpha compatibility anchors, and representative Full V1 CLI surfaces, then writes per-command JSON reports plus static `bundle.json` / `index.html` viewer outputs
  - together they validate parser acceptance/rejection, path-aware unresolved import rejection, expression/statement spans, typed IR lowering, type/scope/import/effect/failure/capability rows, source-derived safe C-like execution, transition/effect rows, generated package-manifest projections for the 12-row source-first operational roots, compute trace shape, effect-session summary shape, static/runtime rejection split, bounded WorldCore observer-safe bootstrap, bounded MembershipChat Mir-owned room-message transform, bounded Sugoroku roll/publish/witness/handoff/local-cut rows, bounded PortalWorldLink resolve/admit/fallback rows, bounded TwoShardHardBoundary offer/prepare/commit rows plus observer-visible old-owner/stale-config reject-event narration around the enforced `missing_live_witness` row, bounded observer-only GradientObservation view/hint rows plus observer-visible write-reject/stale-view-drop narration around the enforced freshness `contract_require_failed` row, same-client same-observation-snapshot no-split-frame, anchor-switch frontier monotonicity, stale-anchor membership rejection, fallback-only reacquire requirement, bounded pose-aware save/load admissibility, observer-safe PoseGraph/devtools export, source-derived target manifests, packet/FFI schema generation, preservation reports, client-write authority rejection, payload-shape mismatch rejection, same-shape heterogeneous effect-contract rejection, same-binary local role-run, undeclared-entry rejection, bounded provider admission, bounded renderer pose delivery with matching binding_context plus snapshot frontier, disabled-native default preservation, and sample/expected matrix consistency without changing the Product Alpha `package.mir.json` front door
  - they do not claim final public grammar, final typed IR API, final effect ABI, final packet/FFI transport semantics, final server/client binary split, arbitrary native/WASM execution, distributed cut/save execution, final public viewer/devtools family completion, final installer/archive hardening, or package artifact generation
- Surface Mir helper family has a P-SURF-01 parser floor, P-SURF-02 indexed-state semantic checker floor, P-SURF-03 elaboration evidence floor, P-SURF-04 generated communication evidence floor, P-SURF-05 role admission evidence floor, P-SURF-06 source patch hot-plug evidence floor, P-SURF-07 source operational evidence floor, P-SURF-08 static devtools diagnostics evidence floor, and P-SURF-99 final audit closeout
  - `python3 scripts/surface_mir_samples.py matrix --format json`
  - `python3 scripts/surface_mir_samples.py check-all --format json`
  - `python3 scripts/surface_mir_authoring_check.py check-all --format json`
  - `python3 scripts/surface_mir_release_check.py --format json plan --out /tmp/mirrorea-surface-release`
  - `python3 scripts/surface_mir_release_check.py --format json check-all --out /tmp/mirrorea-surface-release`
  - `cargo test -p mir-ast --test surface_mir_parser -- --nocapture`
  - `cargo test -p mir-semantics --test indexed_state_semantics -- --nocapture`
  - `cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture`
  - `cargo test -p mir-semantics --test role_admission_capability_grant -- --nocapture`
  - `cargo test -p mir-runtime --test source_patch_hotplug -- --nocapture`
  - `cargo test -p mirrorea-cli --test surface_mir_cli -- --nocapture`
  - this exercises `samples/full-system-v1-surface/syntax/` through `crates/mir-ast::surface_alpha` and the `surface_mir_alpha_parse` example
  - this also exercises `samples/full-system-v1-surface/indexed-state/` through `crates/mir-semantics::surface_indexed_state` and the `surface_indexed_state_check` example
  - this also exercises `samples/full-system-v1-surface/elaboration/` through `crates/mir-semantics::surface_to_core_elaboration` and the `surface_to_core_elaborate` example
  - this also exercises `samples/full-system-v1-surface/role-admission/` through `crates/mir-semantics::surface_role_admission` and the `surface_role_admission_check` example
  - this also exercises `samples/full-system-v1-surface/source-patch/` through `crates/mir-runtime::surface_source_patch_hotplug` and the `mirrorea-alpha` source command family
  - this also exercises `samples/full-system-v1-surface/devtools/` through static observer-safe devtools bundle projections
  - this also exercises `samples/full-system-v1-surface/{world-core,membership-chat,sugoroku-world,portal-worldlink,two-shard-hard-boundary,gradient-observation}/` through per-row required alpha checks recorded in `operational-matrix.json`
  - current actualized rows are `SURF-01..09`: canonical `S { ... }` accepted, `S[ ... ]` rejected with `bracket_place_scope_not_supported`, record literal accepted, ambiguous brace rejected, role-instance block accepted, undeclared block heads rejected, invalid role binder rejected, and role named `S` disambiguated
  - current actualized indexed-state rows are `IDX-01..05`: S-owned Participant-indexed state accepted, key write without authority rejected, stale key rejected, retained-savepoint compaction rejected, and nested place blocks rejected as ambient authority switches
  - current actualized elaboration rows are `ELAB-01..16`: cross-locus read/write remote requests, RHS indexed-read dependency rows for remote writes, MessageEnvelope rows, visible field publish/observe rows, generated edges, source spans, obligations, read/write/visibility/non-visibility/capability/route/membership underdeclared generated failure-row rejection with LAB-only `lab_diagnostic_details` including request / failure-row context and non-final OBL-024 `diagnostic_soundness_projection` evidence for `ELAB-04/07/10/13..16`, `E-ROW-002` / `VisibilityDenied` repair-bearing evidence for `ELAB-10`, one exact non-final `ELAB-07` set-insertion repair payload, private/non-visible field rejection, unsupported-statement rejection, and nested-place read placement
  - current actualized role-admission rows are `ROLE-01..04`: role claim, join admission request, accepted verdict, capability grant, admission witness, missing-grant write rejection, stale membership rejection, and optional hash metadata without safety-proof claims
  - current actualized source-patch rows are `PATCH-01..04`: accepted visible-state patch activation, undeclared generated failure-row rejection, self-grant rejection, lifecycle/devtools activation evidence, HotPlugRequest / HotPlugVerdict / Core IR diff / activation_cut rows, no direct eval, and rejected-without-mutation evidence
  - current actualized devtools rows are `DEV-01..02`: required Surface source, generated Core IR, semantic-checker-backed indexed-state map, generated communication, role/admission, redacted patch lifecycle, and source-span panels with private-field diagnostics and no final viewer/telemetry ABI claim
  - current actualized source operational rows are `E2E-SURF-01..12`: WorldCore, MembershipChat, Sugoroku, PortalWorldlink, TwoShardHardBoundary, and GradientObservation positive/negative `.mir` source rows
  - the P-SURF-99 release-check surface also reruns Product Alpha release check, operational product samples, and the minimal alpha-1 pattern verifier as compatibility anchors
  - this helper floor does not claim runtime MessageEnvelope dispatch, production identity provider, hardware attestation, WAN admission, final source patch ABI, final operational runtime/transport, final devtools viewer/telemetry ABI, distributed durable migration, final grammar/API, or package artifact authority
- practical alpha-1 first hot-plug floor now has an alpha-local script surface
  - `python3 scripts/practical_alpha1_attach.py check-all --format json`
  - this exercises `samples/practical-alpha1/packages/hp-a1-*/` through `crates/mir-ast::practical_alpha1_hotplug_plan` and `crates/mir-runtime::practical_alpha1_hotplug`
  - it consumes checked practical package input through a distinct hotplug-plan boundary
  - current actualized rows are `HP-A1-01..05`、`HP-A1-04B1`、`HP-A1-04B2`、`HP-A1-06`、`HP-A1-07`
  - `HP-A1-07` is an explicit deferred detach minimal contract boundary with `operation_kind = detach` and `detach_boundary_ref`
  - it is a non-final hot-plug command and does not claim final object package attach, detach runtime lifecycle, Docker transport, save/load, or final public package/hot-plug API
- practical alpha-1 first transport floor now has an alpha-local script surface
  - `python3 scripts/practical_alpha1_transport.py check-all --format json`
  - this exercises `samples/practical-alpha1/packages/tr-a1-*/` through `crates/mir-ast::practical_alpha1_transport_plan` and `crates/mir-runtime::practical_alpha1_transport`
  - it consumes checked practical package input through a distinct transport-plan boundary
  - current actualized rows are `TR-A1-01..07`
  - `TR-A1-02` uses `samples/practical-alpha1/docker/docker-compose.practical-alpha1.yml` to run a world server plus participant client over Docker Compose TCP
  - it is a non-final transport command and does not claim WAN/federation, save/load, devtools export, product prototype, or final public transport API
- practical alpha-1 first devtools export floor now has an alpha-local script surface
  - `python3 scripts/practical_alpha1_export_devtools.py check-all --format json`
  - this exercises `VIS-A1-01/02/03/04/05/06/07` over exact practical local-runtime / save-load / hotplug / transport / avatar-preview reports
  - it consumes exact practical reports through a distinct devtools export bundle boundary
  - current actualized rows are `VIS-A1-01/02/03/04/05/06/07`
  - `VIS-A1-03` consumes exact `SL-A1-02` save-load evidence while preserving the saved frontier, later live membership advance, restored frontier, and stale-membership reject
  - `VIS-A1-05` consumes exact `AV-A1-03` fallback evidence while preserving the rejected source lane, degraded roles, and missing host capability
  - `VIS-A1-07` consumes exact `SL-A1-02` retained-artifact evidence while preserving report-local artifact ids, fetch selectors, and hit/miss query outcomes
  - `render-html` emits a non-final static HTML viewer file over the same bundle payload
  - it is a non-final devtools command and does not claim full devtools completion, distributed durable membership timeline, detach runtime lifecycle execution, durable retained-artifact catalog service, cross-session/remote retrieval, retention expiry lifecycle, save/load, product prototype, native execution, unsupported-runtime execution success, or final public viewer/telemetry API
- practical alpha-1 first local save/load floor now has an alpha-local script surface
  - `python3 scripts/practical_alpha1_save_load.py check-all --format json`
  - this exercises `samples/practical-alpha1/packages/sl-a1-*/` through `crates/mir-ast::practical_alpha1_save_load_plan` and `crates/mir-runtime::practical_alpha1_save_load`
  - it keeps 2 branches separate:
    - runtime-backed `SL-A1-01/02` consume checked practical package input, one exact practical local-runtime frontier, and a distinct save-load plan boundary before building a saved local frontier and a non-final save-load report
    - checker-backed `SL-A1-03` lowers an exact rejected checker report into a distinct save-load preflight reject report before any saved local frontier is built
  - current actualized rows are `SL-A1-01/02/03`
  - `CHK-CUT-01` is reused only as an orphan-receive checker guard for the preflight reject branch
  - it is a non-final save/load command and does not claim distributed durable save/load, stale witness/stale lease non-resurrection completion, queue/channel/transport persistence, product prototype, or final public save-load API
- practical alpha-1 first avatar preview companion floor now has an alpha-local script surface
  - `python3 scripts/practical_alpha1_avatar.py check-all --format json`
  - this exercises `samples/practical-alpha1/packages/av-a1-*/` through `crates/mir-runtime::practical_alpha1_avatar`
  - it consumes checked practical package input through a distinct hotplug-plan boundary and exact hot-plug source reports
  - current actualized rows are `AV-A1-01/02/03`
  - `AV-A1-02` is a non-final custom Mir avatar preview with `native_execution_performed = false`
  - `AV-A1-03` keeps the source hot-plug report rejected for missing host capability and lowers only a visible monotone placeholder fallback preview
  - it is a non-final avatar-preview command and does not claim native execution, final avatar package ABI, same-session product runtime completion, active runnable-root promotion, or VRM / VRChat / Unity compatibility
- practical alpha-1 first product-preview floor now has an alpha-local script surface
  - `python3 scripts/practical_alpha1_product_preview.py check-all --format json`
  - this exercises `samples/practical-alpha1/previews/` through preview manifests over exact practical runtime / hot-plug / transport / save-load reports, exact avatar preview companion reports, and exact practical devtools bundles
  - current actualized rows are `PE2E-01..09`
  - `render-html` emits a non-final static HTML preview over the same exact bundle payloads
  - `PE2E-04` is narrowed to `HP-A1-06` placeholder object preview companion evidence only
  - `PE2E-06` consumes exact `SL-A1-03` save-load preflight reject evidence rather than direct checker evidence
  - `PE2E-08` consumes `AV-A1-02` as a custom-avatar companion preview bundle with `native_execution_performed = false`
  - `PE2E-09` consumes `AV-A1-03` as an unsupported-runtime visible fallback companion preview bundle while the source avatar lane remains rejected
  - it is a non-final product-preview command and does not claim native execution, same-session runtime attach/detach execution, unsupported-runtime execution success, active runnable-root promotion, operational α-0.5 / α-0.8 / α-0.9 completion, or final public CLI / viewer / transport / save-load / package-avatar API

### current-L2 helper / detached loop / support

- `current_l2_*`
  current-L2 source corpus、detached validation loop、diff/export assist、Lean sync、checker support
- `current_l2_lean_sample_sync.py`
  active Lean foundations、LAB statement drafts、clean-near-end generated
  theorem stubs を検証し、`samples/lean/manifest.json` に同期する。
  `statement_drafts` は compile-check only であり、proof discharge や canon
  OBL status ではない。
- `current_l2_model_check_carrier_pipeline.py`
  current-L2 authored source sample の formal-hook smoke から model-check carrier emit までを確認する repo-local conformance helper。production model checker binding ではない。
- `new_report.py`
  report utility
- alpha-specific helper/runner family は mixed 状態で actualize 済み
  - `alpha_lifetime_fallback_checker.py`、`alpha_contract_variance_checker.py`、`alpha_cut_save_load_checker.py` は current first checker-floor helper として actualize 済み
  - これは selected `samples/alpha/` sidecar の `expected_static.checked_reason_codes` と synthetic detached artifact を照合する non-public helper であり、shared support は `current_l2_family_checker_support.py` を reuse する。現時点の row inventory は `LIF-01/05..08`、`VAR-02/03/05/07/09/10/15`、`CUT-05/07/08/09/11/13/14/15` で、artifact 側 `reason_codes_scope` が family floor（Alpha は `alpha-static-floor`）と一致しない row は matched 扱いしない。parser/runtime integration ではない
  - `alpha_lifetime_fallback_acceptance.py` と `alpha_contract_variance_acceptance.py` は helper-local synthetic acceptance floor として actualize 済み
  - これは selected positive sidecar の `expected_acceptance.checked_acceptance_rows` と synthetic detached artifact の `detached_noncore.acceptance_rows` を照合する non-public helper であり、shared support は `current_l2_family_acceptance_support.py` を使う。現時点の row inventory は `LIF-02/03/04` と `VAR-01/04/06` で、artifact 側 `acceptance_scope` が family floor（Alpha は `alpha-acceptance-floor`）と一致しない row は matched 扱いしない。negative `reason_codes` helper と parser/runtime integration ではない
  - `alpha_lifetime_fallback_snapshot.py` は selected snapshot-selected positive row の helper-local snapshot floor として actualize 済み
  - これは sidecar の `expected_snapshot.checked_snapshot_rows` と synthetic detached artifact の `detached_noncore.snapshot_rows` を照合する non-public helper であり、shared support は `current_l2_family_snapshot_support.py` を使う。現時点の row inventory は `LIF-13` だけで、artifact 側 `snapshot_scope` が family floor（Alpha は `alpha-snapshot-selected-floor`）と一致しない row は matched 扱いしない。acceptance row、negative `reason_codes` helper、parser/runtime integration ではない
  - `alpha_lifetime_fallback_anchor_handoff.py` は selected anchor-handoff positive row の helper-local anchor-handoff floor として actualize 済み
  - これは sidecar の `expected_anchor_handoff.checked_anchor_handoff_rows` と synthetic detached artifact の `detached_noncore.anchor_handoff_rows` を照合する non-public helper であり、shared support は `current_l2_family_anchor_handoff_support.py` を使う。現時点の row inventory は `LIF-11` だけで、artifact 側 `anchor_handoff_scope` が family floor（Alpha は `alpha-anchor-handoff-floor`）と一致しない row は matched 扱いしない。reason-code helper、acceptance helper、snapshot helper、parser/runtime integration ではない
  - `alpha_contract_variance_runtime_mirror.py` は selected runtime-sensitive positive rows の runtime-mirror floor として actualize 済み
  - これは target sidecar の `runtime_mirror` と existing source runtime-floor sidecar を照合する non-public helper であり、shared support は `current_l2_family_runtime_mirror_support.py` を使う。現時点の row inventory は `VAR-08/11/13` で、target 側 `runtime_mirror.scope` が family floor（Alpha は `alpha-runtime-mirror-floor`）と一致しない row は matched 扱いしない。`reason_codes` helper、`acceptance_rows` helper、parser/runtime bridge とは別 carrier である
  - `P-A0-07` local-runtime first cut と `P-A0-08` layer-insertion first cut は `scripts/` ではなく `crates/mir-runtime` の `alpha_local_runtime` / `alpha_layer_insertion_runtime` modules, examples, and integration tests に actualize している。current sample identity anchors は `samples/alpha/local-runtime/` と `samples/alpha/layer-insertion/` だが、`.mir` files are still source-ish placeholders rather than parsed inputs
  - `P-A0-23` は `alpha_local_runtime_samples.py` を `scripts/` に actualize した。これは `samples/alpha/local-runtime/` の `LR-01/02` を dedicated sample-ID keyed runner として検証し、`stage-b-closeout` command では `CUT-04/17` を local-only save/load supporting subset として再利用して current-scope Stage B closeout を示す。distributed save/load completion、active runnable-root promotion、parser/runtime bridge は主張しない
  - `P-A0-09` は `crates/mir-runtime/src/alpha_network_runtime.rs` と example `mirrorea_alpha_network_runtime` を主体にしつつ、thin Docker runner `alpha_network_docker_e2e.py` を `scripts/` に actualize した。これは `samples/alpha/network-docker/` の `NET-02/03/04/05/07/09` を narrow local-container / TCP bridge cut として検証するもので、helper-local `network_transport_samples.py` の canary familyを置き換えない
  - `P-A0-24` は `alpha_network_docker_e2e.py` に `stage-c-closeout` surface と sidecar-backed narrow contract checks を追加し、`NET-02/03/04/05/07/09` を current-scope Stage C closeout set として束ねる。`NET-06/08/10`、production WAN/session/replay、network partition completion、final public transport ABI は主張しない
  - `P-A0-10` は `crates/mir-runtime/src/alpha_avatar_runtime.rs` と example `mirrorea_alpha_avatar_runtime` を主体にしつつ、thin runner `alpha_avatar_runtime_samples.py` を `scripts/` に actualize した。これは `samples/alpha/avatar-runtime/` の `AV-01/02/06/08/09` と `samples/alpha/hotplug-runtime/` の `HP-11/12/15` を runtime-private package/avatar admission floor として検証するもので、final avatar API / native execution / hot-plug lifecycle completion を主張しない
  - `P-A0-25` は `alpha_hotplug_lifecycle_samples.py` を `scripts/` に actualize した。これは `samples/alpha/layer-insertion/` の `LI-01/02/03/04/05` と `samples/alpha/avatar-runtime/` / `samples/alpha/hotplug-runtime/` の `AV-01/02/06/08/09` / `HP-11/12/15` を current-scope Stage D closeout surface として束ねるもので、detach runtime、durable migration、distributed activation ordering、native execution realization、final public layer/package/avatar ABI は主張しない
  - `P-A0-11` / `P-A0-27` は thin integrated bridge runner `alpha_e2e_samples.py` を `scripts/` に actualize / widen した。これは `samples/alpha/e2e/` の `E2E-01/02/03/04/05/06/07/09/10` を既存 Stage B/C/D/E subset floor の composition として検証し、`stage-f-closeout` で current-scope Stage F closeout surfaceを与える。`E2E-08`、public alpha / `U1` completion、active runnable-root promotion は主張しない
  - `P-A0-12` は `alpha_cut_save_load_samples.py` を `scripts/` に actualize した。これは `samples/alpha/cut-save-load/` の `CUT-04` local-only save/load bridge を専用 command として検証し、`alpha_e2e_samples.py` 側では `E2E-06` へ composition する。distributed/durable save/load completion や Z-cycle handling は主張しない
  - `P-A0-13` / `P-A0-15` / `P-A0-26` は `alpha_visualization_samples.py` を widen し、`samples/alpha/visualization/` の `VIS-01/02/03/05/06/07/08/10/11` を existing alpha/helper/runtime JSON evidence の dedicated Stage-E subset runner として検証し、`stage-e-closeout` command で current-scope Stage E completion surface を与える。`VIS-04/09/12`、Stage F completion、final public viewer/telemetry API は引き続き主張しない
  - `P-A0-14` は `alpha_cut_save_load_samples.py` と `alpha_cut_save_load_checker.py` を widen し、`CUT-17` local stale-membership rejection bridge と `CUT-11` checker-backed Z-cycle inadmissibility row を actualize した。これは saved local state が stale membership を accepted resumed dispatch へ resurrect しないことと、useless checkpoint cycle が checker floor で inadmissible であることだけを示す。`CUT-10/12/16`、distributed/durable save/load completion、Z-cycle repair、final public ABI は主張しない
  - practical alpha-1 front-door script surface は staged に actualize している。`P-A1-02` で `practical_alpha1_check.py`、`P-A1-03` で `practical_alpha1_run_local.py`、`P-A1-04a..c` で `practical_alpha1_attach.py`、`P-A1-05` で `practical_alpha1_transport.py`、`P-A1-06` で `practical_alpha1_export_devtools.py`、`P-A1-07` で `practical_alpha1_save_load.py`、`P-A1-23` で `practical_alpha1_integrated_workflow.py` が widened された
  - `P-A1-06` と `P-A1-09` と `P-A1-12` と `P-A1-13` と `P-A1-15` では event DAG export、observer-safe route trace export、membership timeline export、exact-report hot-plug lifecycle export、fallback degradation export、redacted observer view、report-local retention query export を distinct devtools bundle + non-final viewer surface として actualize した。`VIS-A1-07` は exact `SL-A1-02` save-load report に widened した report-local retained-artifact catalog と hit/miss query trace だけを consume し、durable retained-artifact service や remote retrieval semantics は追加しない
  - `P-A1-07` と `P-A1-16` では `SL-A1-01/02/03` を widened practical save-load floor として actualize した。runtime-backed branch は distinct save-load plan + saved local frontier + non-final save-load report surface、checker-backed branch は exact rejected checker report -> distinct save-load preflight reject report surface を保つ。distributed durable save/load、stale witness/stale lease non-resurrection completion、queue/channel/transport persistence、product command は still later である

### storage / env

- `env/`
- `env/mirrorea_storage_env.sh`
  mounted external workdir 前提の env export surface。`MIRROREA_WORKDIR`、`CARGO_TARGET_DIR`、`CARGO_HOME`、LLVM staging path、mount/ownership/writable status を返し、`--ensure-dirs` は unmounted default root への heavy dir 作成を拒否する
- `storage/`
- `storage/setup_mirrorea_workdisk_root.sh`
  mount / fstab / symlink / ownership repair を伴う one-time root setup path。routine helper ではない
- `storage/detach_prepare.sh`
  non-destructive storage audit。disk / mount / repo usage / external workdir usage / LLVM staging dir ownership / disposable candidates を確認する
- `storage/cleanup_disposable_artifacts.sh`
  explicit `--confirm` 必須の disposable cleanup helper。known disposable dir だけを対象にし、`llvm/src` は意図的に対象外、`llvm` parent が non-writable な場合の build/install cleanup も拒否する

### tests

- `tests/`

## reading rules

- active repo-local command path は上記 front-door runner を先に使う
- `current_l2_guided_samples.py` は current-L2 front-door compatibility path であり、legacy bundle commands は持たない
- `current_l2_*` helper 群は public installed CLI ではなく repo-local support surface として読む
- `samples/alpha/` 向けの future runner 名は roadmap / sample matrix にだけ置き、実在しない command を current validation floor に入れない
- `alpha_network_docker_e2e.py` は current actualized command だが、active clean-suite front door ではなく Alpha-0 package closeout evidence command として読む
- `alpha_avatar_runtime_samples.py`、`alpha_visualization_samples.py`、`alpha_e2e_samples.py` も active clean-suite front door ではなく Alpha-0 package closeout evidence command として読む
- storage / env script は root setup と cleanup policy を helper 本体から分離する

## staged reorganization policy

- いまは flat layout を維持する
- future に `samples/`, `validation/`, `docs/`, `visualization/` などへ rebucket する可能性はある
- ただし active alpha command を壊す move は、wrapper / alias なしでは行わない
