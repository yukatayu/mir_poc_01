# scripts

この directory は、**active runner、repo-local helper、detached/export assist、storage/env、tests** を置く。

## current taxonomy

### front-door checks and active runners

- `check_source_hierarchy.py`
  required root docs / specs / plan / support directory が存在するかを見る structural check。current line では `specs/13..25`、`plan/39..50`、`samples/alpha/`、`samples/product-alpha1/README.md`、`samples/product-alpha1/demo/README.md`、`samples/product-alpha1/demo/package.mir.json`、`sub-agent-pro/alpha-0/`、`sub-agent-pro/alpha-1/` も structural presence の対象に入る。文書内容、stale wording、normative consistency、report template completeness は判定しない。
- `validate_docs.py`
  required documentation scaffold、numbered report、report template closeout headings、latest numbered report の required heading presence / order、empty required section、unresolved update-status placeholder を確認する scaffold check。current line では snapshot docs、`samples/README.md` / `scripts/README.md`、`samples/alpha/README.md`、`samples/product-alpha1/README.md`、`samples/product-alpha1/demo/README.md`、`samples/product-alpha1/demo/package.mir.json`、`plan/39..50`、`specs/13..25` も required scaffold に入る。historical report 全体の semantic validation、active/current wording lint、sample execution、Cargo validation は別 command の責務。
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
  - it actualizes same-session `start` / `observe` / `save` / `load` over the bounded local runtime carrier plus session-bound event DAG / observer-safe export and one minimal typed external `AddOne` direct execution lane
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
  - this exercises `samples/product-alpha1/demo/package.mir.json` through `crates/mir-ast::product_alpha1`
  - `check` is implemented for schema acceptance / explicit residual obligations
  - `run-local`、`session`、`attach`、`save`、`load`、`quiescent-save` are implemented for a local file-backed product session carrier through `crates/mir-runtime::product_alpha1_session`
  - `transport`、`export-devtools`、`view`、`build-native-bundle`、`demo` currently return structured unsupported diagnostics
  - it does not claim local/Docker transport command completion, product-ready alpha-1, final public CLI/API, direct `.mir` grammar, WAN/federation, distributed durable save/load R3/R4, viewer completion, native bundle completion, or arbitrary native execution
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
