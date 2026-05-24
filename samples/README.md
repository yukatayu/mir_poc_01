# samples

この directory は、**active sample、base corpus、planned skeleton、prototype reference、archive、generated reserve** を分けて置く。

## current taxonomy

- `clean-near-end/`
  active canonical executable suite
- `current-l2/`
  base current-L2 source corpus
- `lean/`
  mechanization evidence
- `alpha/`
  Mirrorea Spaces alpha-0 phase-indexed sample matrix scaffold with `.expected.json` sidecars
  - current-scope evidence root であり、practical alpha-1 front-door root ではない
  - Stage A..F `100%` は evidence closeout として読む
- `practical-alpha1/`
  practical alpha-1 front-door sample root
  - current cuts は limited `package.mir.json` front-door fixture family、first checker-floor fixture family、`RUN-01..04` を含む first local-runtime fixture family、attach-time freshness/witness negatives、narrow object preview seam、explicit deferred detach minimal contract を含む practical hot-plug fixture family、exact hot-plug reports を source とする `AV-A1-01/02/03` practical avatar preview fixture family、distinct transport-plan/report carrier を通る `TR-A1-01..07` practical transport fixture family、exact practical reports を source とする `VIS-A1-01/02/03/04/05/06/07` practical devtools export bundle family、runtime-backed `SL-A1-01/02` と checker-backed preflight `SL-A1-03` を含む practical local save/load fixture family、preview manifest から exact practical reports / exact practical devtools bundles / exact avatar preview reports を束ね、`PE2E-06` では exact `SL-A1-03` preflight reject report を consume する `PE2E-01..09` practical product-preview family、そして `PA1W-01..08` bounded practical integrated workflow carrier
  - active canonical runnable root や full toolchain root ではまだない
- `product-alpha1/`
  product/public alpha-1 sample root
  - `demo/` is the P-A1-26 versioned schema / CLI fixture root, P-A1-27 local same-session runtime fixture root, P-A1-28 local save/quiescent-save fixture root, P-A1-29 transport/viewer fixture root, P-A1-30 native host launch bundle fixture root, and P-A1-31 release-candidate demo root
  - `operational/` is the `P-OPS-01` canonical operational product sample suite root with `WorldCore -> MembershipChat -> SugorokuWorld -> PortalWorldLink -> TwoShardHardBoundary -> TwoShardGradientObservation`, the bounded `portal-worldlink/`, `two-shard-hard-boundary/`, and `two-shard-gradient-observation/` roots, shared attach packages, deployment/projection intent, retained portal/shard blueprint inventory, the `P-OPS-09` / `P-OPS-10` template-only authoring starter catalog under `operational/templates/`, the `P-OPS-11` docs-first `future/gradient-observation.profile.json`, and the `P-OPS-12` decision that keeps portal/shard authoring on active roots rather than starter duplicates
  - backend-feasibility wording for `operational/` stays docs-first: `native host launch bundle` is actualized, while WASM/LLVM remain inventory-only
  - `docker/` holds the controlled Product Alpha-1 Docker Compose TCP fixture used by `mirrorea-alpha transport --mode docker`
  - `cargo run -q -p mirrorea-cli -- demo --out /tmp/mirrorea-alpha1-demo --format json` and `python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release` are the release-candidate validation anchors
  - `python3 scripts/operational_product_samples.py check-all --format json` is the operational suite validation anchor
  - `cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/templates/world-core-starter --format json`, the same command for `membership-chat-starter` and `sugoroku-world-starter`, and `run-local` on those roots validate the current authoring starter catalog, but the templates are not counted as active operational sample roots
  - this is not final public product readiness, WAN/federation, distributed durable save/load R3/R4, arbitrary native package execution, or final public API
- `full-system-v1/`
  source-first Full System V1 sample root under `specs/33..38` / `plan/58..63`
  - `P-MIR-01` actualizes `samples/full-system-v1/computational/` as the first textual Mir alpha grammar lane with 2 positive rows, 8 negative rows, `matrix.json`, `expected/parse.json`, and `python3 scripts/textual_mir_samples.py check-all --format json`
  - `P-MIR-02` adds the first typed checker lane with 3 positive rows, 9 negative rows, `typed-ir-matrix.json`, `expected/check.json`, and explicit accepted/residual obligations
  - `P-MIR-03` adds the first pure interpreter lane with `runtime-matrix.json`, `expected/run.json`, `cargo test -p mir-semantics --test typed_ir_interpreter -- --nocapture`, `cargo test -p mir-runtime --test full_system_v1_session -- --nocapture`, and `python3 scripts/full_system_v1_samples.py check-all --format json`
  - `P-MIR-04` widens the same root to bounded transition/effect rows for host read/write, publish/observe, witness/handoff, and local atomic-cut negatives
  - `P-POSE-03` / `P-POSE-04` actualize `samples/full-system-v1/avatar-pose/` as the bounded runtime PoseGraph lane with 9 executable rows, `matrix.json`, `expected/run.json`, `cargo test -p mir-runtime --test posegraph_runtime -- --nocapture`, and `python3 scripts/posegraph_runtime_samples.py check-all --format json`
  - `P-PROJ-03` actualizes `samples/full-system-v1/projection/` as the bounded projection IR + boundary-schema lane with 1 positive row, 3 negative rows, `matrix.json`, `expected/run.json`, generated `projection-artifacts.json` / `rejection-report.json`, `cargo test -p mir-runtime --test projection_ir -- --nocapture`, `cargo test -p mirrorea-cli --test full_system_v1_cli -- --nocapture`, and `python3 scripts/projection_v1_samples.py check-all --format json`
  - `P-PROJ-04` actualizes `samples/full-system-v1/server-client/` as the bounded same-binary local role-split lane with 1 accepted row, 1 undeclared-entry rejection row, generated `local-split-report.json`, `crates/mir-runtime::full_system_v1_local_split`, and `mirrorea-alpha run-full-v1-split`
  - `P-ENG-02` actualizes `samples/full-system-v1/provider-adapter/` as the bounded provider-admission lane with 2 accepted rows, 3 rejection rows, generated `provider-admission-report.json`, `crates/mir-runtime::full_system_v1_provider_admission`, the `mir_full_system_v1_provider_admission` example, `mirrorea-alpha admit-provider-v1`, and `python3 scripts/provider_admission_samples.py check-all --format json`
  - `P-ENG-03` widens that same root with `renderer-pose-matrix.json`, generated `renderer-pose-backend-report.json`, `crates/mir-runtime::full_system_v1_renderer_pose_backend`, the `mir_full_system_v1_renderer_pose_backend` example, `mirrorea-alpha render-pose-backend-v1`, and `python3 scripts/renderer_pose_backend_samples.py check-all --format json` so one accepted row and two blocked rows prove renderer receipt of a matching binding-context plus snapshot-frontier delivery without widening semantic ownership or claiming attested package provenance
  - `P-FSV1-01` and `P-FSV1-02` actualize `samples/full-system-v1/world-core/`, `membership-chat/`, `sugoroku-world/`, `portal-worldlink/`, `two-shard-hard-boundary/`, and `gradient-observation/` as bounded source-first operational roots with 12 executable rows, generated package-manifest expectations, runtime report expectations, explicit `missing_publication` / `contract_require_failed` / `missing_live_witness` negatives, bounded WorldCore observer-safe bootstrap evidence, bounded MembershipChat Mir-owned room-message transform evidence, bounded Sugoroku roll/publish/witness/handoff/local-cut evidence, bounded PortalWorldLink resolve/admit/fallback evidence, bounded TwoShardHardBoundary offer/prepare/commit evidence plus observer-visible old-owner/stale-config reject-event narration around the enforced `missing_live_witness` negative, and bounded observer-only GradientObservation view/hint evidence plus observer-visible write-reject/stale-view-drop narration around the enforced freshness `contract_require_failed` negative
  - current executable scope is parser plus typed checker plus bounded effectful runtime plus bounded PoseGraph runtime/save-load/devtools evidence plus bounded projection IR/boundary-schema evidence plus bounded local role-split evidence plus bounded provider admission plus bounded source-first WorldCore/MembershipChat/Sugoroku/Portal/TwoShard/Gradient evidence plus the bounded line-level release-check/report/viewer bundle lane; it does not claim final effect grammar, final packet/FFI transport semantics, final server/client binary split, final provider ABI, final public viewer/devtools family completion, or generated package artifact completion
  - these roots remain evidence-closed rather than individually workflow-ready; `P-FSV1-03` adds a line-level bounded release-check workflow via `scripts/full_system_v1_release_check.py`, and `P-FSV1-99` closes the bounded chain through final audit without widening final-product claims
  - planned `gradient-observation/` is the Full System V1 source-first counterpart to the Product Alpha operational runtime root `two-shard-gradient-observation/`
  - `samples/full-system-v1/projection/` is `FS-06` projection IR evidence only; `samples/full-system-v1/server-client/` is the separate `FS-07` local server/client role-run root
  - `samples/full-system-v1/provider-adapter/` is the separate `FS-08` bounded provider-admission plus renderer-pose-backend root
  - do not mark the whole root workflow-ready until later packages add broader source-first operational families beyond the current bounded release-check closure
- `full-system-v1-surface/`
  Surface Mir source-authority sample root under `specs/39..43` / `plan/64..68`
  - `syntax/` is actualized by `P-SURF-01` as parser evidence for `SURF-01..09`
  - `python3 scripts/surface_mir_samples.py check-all --format json` validates canonical `S { ... }`, `S[ ... ]` rejection, record literal disambiguation, ambiguous brace rejection, role-instance block parsing, namespace-head rejection, invalid role binders, and role named `S` disambiguation
  - `indexed-state/` is actualized by `P-SURF-02` as semantic checker evidence for `IDX-01..05`
  - the same helper validates S-owned Participant-indexed state, key-not-authority rejection, stale-key rejection, retained-savepoint compaction rejection, and nested-place ambient-authority rejection
  - planned future families are `world-core/`, `membership-chat/`, `sugoroku-world`, `role-admission/`, `patch-hotplug/`, `posegraph/`, `projection/`, and `provider/`
  - the root is not workflow-ready runtime evidence until later `P-SURF-*` packages add elaboration, runtime, and operational rows
  - canonical place-scope syntax for future source samples is `S { ... }`; `S[ ... ]` is rejected and is not a compatibility sugar
  - `package.mir.json` remains alpha artifact / compatibility output, not semantic source authority
- planned future product-alpha1 semantic roots
  - `samples/product-alpha1/computational/` is the Mir-owned computation line under `specs/28` / `plan/53`
    - `python3 scripts/mir_computational_samples.py check-all --format json` validates one direct `add_one` runtime row, five accepted helper rows, five expected runtime rejection rows, one direct host-I/O boundary accepted row, and three expected `check` rejections
    - `run comp-02-pure-add-one` executes the bounded `ReadInt -> add_one -> WriteInt` path
    - `run comp-03-control-flow-positive` and `run comp-03-variables-scope-negative` execute helper-package first-floor evidence rather than the direct product-alpha CLI surface
    - `run comp-04-host-io-internal-transform-positive` and `run comp-04-host-io-internal-transform-negative-undeclared-effect` prove the bounded host read/write boundary line without claiming broader effectful semantics
  - `scripts/minimal_alpha1_patterns.py check-all --format json` is the compact strict verifier for the current minimal alpha-1 pattern set:
    - it checks exact computational / PoseGraph / projection / engine-adapter row counts and expected rejection IDs
    - it lists product release-candidate and operational Sugoroku as workflow anchors without silently turning those anchors into final product claims
    - use `--include-workflows --out <dir>` only when the heavier product / operational workflow anchors should be rerun in the same helper
  - `samples/product-alpha1/posegraph/` is the `P-POSE-02` bounded Transform / PoseGraph helper evidence line under `specs/29` / `plan/54`
    - `python3 scripts/posegraph_samples.py check-all --format json` validates the matrix, keeps 7 rows `planned_only`, and confirms 1 accepted row plus 1 violation-export row
    - `run pose-04-no-split-frame-positive` returns helper-backed accepted same-snapshot evidence and `run pose-05-split-frame-negative` returns helper-backed `violation_export`
    - this is not full PoseGraph runtime completion, save/load admissibility, or devtools panel completion
  - `samples/product-alpha1/projection/` is the `P-PROJ-01` planned-only projection boundary inventory scaffold under `specs/30` / `plan/55`
    - `python3 scripts/projection_boundary_samples.py check-all --format json` validates the matrix and keeps every row `planned_only`
    - `run proj-01-server-client-target-manifest` must reject as `planned_only` until a later projection-realization package exists
  - `samples/product-alpha1/engine-adapter/` is the `P-ENG-01` planned-only engine / WASM / FFI adapter inventory scaffold under `specs/31` / `plan/56`
    - `python3 scripts/engine_adapter_boundary_samples.py check-all --format json` validates the matrix and keeps every provider row `planned_only`
    - `run wasm-sandbox` must still reject as `planned_only`; bounded provider-admission evidence now lives under `samples/full-system-v1/provider-adapter/`
    - do not mark these roots workflow-ready until real check / run / observe / negative evidence exists
- `not_implemented/`
  residual planned skeleton family
- `prototype/`
  historical prototype / compatibility anchor
- `old/`
  archive
- `generated/`
  non-Lean generated sample artifact reserve + committed generated bridge evidence

## reading rules

- active executable sample として読むのは、まず `clean-near-end/`
- `current-l2/` は active clean suite を支える base source corpus
- `lean/clean-near-end/` は generated theorem stub だが、proof bridge evidence として committed されている
- `alpha/` は current alpha-local theory-freeze / checker/runtime roadmap 用の scaffold であり、expected-verdict sidecar を伴う
  - current task では mixed alpha-local scaffold root として扱い、active runnable sample root に silently promote しない
  - root 内には scaffold-only family と runner-backed non-public floor が併存するが、いずれも repo の active canonical root ではない
  - `lifetime-fallback/` は `LIF-01/05..08`、`contract-variance/` は `VAR-02/03/05/07/09/10/15` について `reason_codes_scope = alpha-static-floor` の non-public checker-floor seed rows を current repo state で持つ
  - `lifetime-fallback/` の `LIF-02/03/04` と `contract-variance/` の `VAR-01/04/06` は `acceptance_scope = alpha-acceptance-floor` の helper-local synthetic acceptance rows を current repo state で持つ
  - `lifetime-fallback/` の `LIF-13` は `snapshot_scope = alpha-snapshot-selected-floor` の helper-local synthetic snapshot-selected row を current repo state で持つ
  - `lifetime-fallback/` の `LIF-11` は `anchor_handoff_scope = alpha-anchor-handoff-floor` の helper-local synthetic anchor-handoff row を current repo state で持つ
  - `contract-variance/` の `VAR-08/11/13` は `runtime_mirror.scope = alpha-runtime-mirror-floor` の non-public runtime-mirror rows を current repo state で持ち、source authority は `layer-insertion/` の `LI-04/01/03` runtime-floor sidecars に残す
  - `reason_codes_scope` confinement、`acceptance_scope` confinement、`snapshot_scope` confinement、`anchor_handoff_scope` confinement、`runtime_mirror.scope` confinement は別であり、positive row は negative reason code 不在ではなく explicit acceptance rows、snapshot rows、anchor-handoff rows、または runtime-mirror rows で扱う
  - current package line では `local-runtime/`、`layer-insertion/`、`network-docker/`、`avatar-runtime/` が non-public Rust/runtime-private floor を持つが、sample file parsing front door ではなく、active root でもない。`local-runtime/` は `alpha_local_runtime_samples.py` により current-scope Stage B closeout surface を持つが、これは `CUT-04/17` local-only save/load subset を supporting evidence として再利用するだけで、distributed save/load や active runnable-root promotion は主張しない
  - `layer-insertion/` と `avatar-runtime/` / `hotplug-runtime/` は `alpha_hotplug_lifecycle_samples.py` により current-scope Stage D closeout surface を持つが、これは `LI-01/02/03/04/05` と `AV-01/02/06/08/09` / `HP-11/12/15` attach-time/package-admission subset を束ねるだけで、detach runtime、durable migration、distributed activation ordering、native execution realization、final public ABI は主張しない
  - `network-docker/` は `mirrorea_alpha_network_runtime` + `alpha_network_docker_e2e.py` により `NET-02/03/04/05/07/09` を narrow Stage-C transport / Docker cut として actualize し、`stage-c-closeout` command が current-scope Stage C closeout surface を与える。helper-local `clean-near-end/network-transport/` canary familyとは別物であり、`NET-01/06/08/10` は引き続き planned
  - `avatar-runtime/` は `mirrorea_alpha_avatar_runtime` + `alpha_avatar_runtime_samples.py` により `AV-01/02/06/08/09` を runtime-private package/avatar admission floor として actualize した。`HP-11/12/15` も同 runner で検証するが、`hotplug-runtime/` family 全体を runnable root へ昇格したわけではない
  - `cut-save-load/` は `alpha_cut_save_load_samples.py` により `CUT-04` local-only save/load bridge、`CUT-17` stale-membership rejection bridge、`CUT-11` checker-backed Z-cycle inadmissibility row を actualize し、selected negative rows の checker floor と併存する。`P-A0-23` 以降は `CUT-04/17` が Stage B closeout supporting subset として再利用されるが、family 全体の 100% 化や `CUT-10/12/16` completion は意味しない
  - `visualization/` は `alpha_visualization_samples.py` により `VIS-01/02/03/05/06/07/08/10/11` を dedicated Stage-E subset runner として actualize し、`stage-e-closeout` command が current-scope Stage E completion surface を与える。`VIS-04/09/12` は planned-only のままであり、family 全体や final viewer API を completion 扱いしない
  - `e2e/` は `alpha_e2e_samples.py` により `E2E-01/02/03/04/05/06/07/09/10` を thin integrated Stage-F bridge として actualize し、`stage-f-closeout` が current-scope Stage F closeout surface を与える。`E2E-08` upper-layer seed は planned-only のまま保ち、public alpha / `U1` completion や active runnable-root promotionには使わない
  - `hotplug-runtime/` と `contract-variance/` の overlapping rows は引き続き planned mirror / verdict authority であり、runtime-sensitive attach evidence authority は `layer-insertion/` 側に置く。`P-A0-18` では `VAR-08/11/13` だけを `LI-04/01/03` から mirror actualization したが、`contract-variance/` 自体を runnable root へ昇格していない。`hotplug-runtime/` では `HP-11/12/15` だけが avatar/package runner 経由の native-policy subset として actualize 済み
  - family ごとの status / blocker / next runner は `samples_progress.md` と `progress.md` / `tasks.md` を参照する
- `not_implemented/` は residual planned skeleton であり、active sample ではない
  - `avatar-fairy-follow/` は phase 8 residual planned family
  - `typed-external-boundary/` は phase 9 residual planned family
    - `scripts/typed_external_boundary_samples.py` はこの root の `EXT-03` / `EXT-04` を helper-local synthetic preview anchor として読むが、active semantic sample root へ昇格させない
  - `network-transport/` は phase 13 planned family
- `clean-near-end/avatar-follow/` は phase 8 active representative slice
- `clean-near-end/network-transport/` は phase 13 active helper-local canary landing page
  - runnable helper-local canary ID は `NET-02` / `NET-03` / `NET-04` / `NET-05`
  - `NET-01` は standalone sample ID ではなく、Sugoroku loopback parity anchor として reported に扱う
- `prototype/` は historical anchor / compatibility reference であり、active canonical path ではない
- `old/` は archive
- `generated/` は reserve path であり、source sample を置かない
  - current committed generated bridge evidence は `samples/generated/projection-placement/manifest.json`
  - これは generated artifact であり、source sample でも final emitted executable program でもない
  - projection / placement の actual emitted executable family、optimizer、deployment planner は kept-later gate に残す
- `not_implemented/` と `alpha/` を混同しない
  - `alpha/` は current alpha-local promoted scaffold
  - `not_implemented/` は residual / historical planned family の preservation root
- `alpha/` と `practical-alpha1/` を混同しない
  - `alpha/` は evidence closeout root
  - `practical-alpha1/` は current repo state では `package.mir.json` loader fixtures、first checker-floor fixtures、`RUN-01..04` local-runtime fixtures、`HP-A1-01..05` / `HP-A1-04B1` / `HP-A1-04B2` / `HP-A1-06` / `HP-A1-07` hot-plug fixtures、`TR-A1-01..07` transport fixtures、`SL-A1-01/02/03` local save/load fixtures を持つ
  - `practical-alpha1/` もまだ active canonical runnable root ではなく、`previews/` root に `PE2E-01..09` の first practical product-preview floor を持ち、別 lane で `AV-A1-01/02/03` の first practical avatar preview floor を持つが、native avatar execution、same-session product runtime、final object package attach、detach runtime lifecycle、WAN/federation、distributed durable save/load、full product surfaces は later packages に残る
  - `scripts/practical_alpha05_session.py` はこの root を first bounded α-0.5 same-session carrier として consume し、`OA05-07` で one minimal typed external `AddOne` host-I/O adapter lane を same-session observer surface へ接続する。これは Mir-owned computational-core evidence ではない。
  - `scripts/practical_alpha08_session_hotplug.py` は同じ root を bounded α-0.8 same-session hot-plug carrier として再利用し、`OA08-01..10` で debug / auth / rate-limit / object preview / deferred detach の accepted/rejected/deferred lifecycle を same-session observer surface へ接続する。rejected attach は active runtime state を mutate しないが、session-carried observation として残る。
  - `scripts/practical_alpha09_devtools.py` は同じ root を bounded α-0.9 session-bound devtools carrier として再利用し、`OA09-01..09` で event DAG / local route trace / membership timeline / witness relation / hot-plug lifecycle / fallback degradation / save-load timeline / observer-safe redacted view / retention-on-demand trace を同じ session export へ接続する。final public viewer / telemetry ABI、durable audit、product-ready alpha-1 は later
  - `scripts/practical_alpha1_integrated_workflow.py` は first-floor exact evidence と bounded operational carriers を 1 つの bounded practical α-1 developer workflow として束ね、`PA1W-01..08` で source front-door、checker、same-session runtime、typed host-I/O、hot-plug、save/load、session devtools、product-preview evidence、negative guard、non-final stop lines を確認する。product/public-ready alpha-1、final public viewer/telemetry ABI、distributed durable save/load は later
  - current practical devtools export floor は `VIS-A1-01/02/03/04/05/06/07` に限られ、`VIS-A1-03` は exact `SL-A1-02` save-load report から saved frontier / later live membership advance / restored frontier / stale-membership reject を export する membership timeline widening、`VIS-A1-04` は exact practical hotplug reports から attach accepted boundary / membership snapshot / deferred detach boundary を export する observability widening、`VIS-A1-05` は exact `AV-A1-03` avatar preview report から rejected source lane / degraded roles / missing host capability を export する fallback degradation widening、`VIS-A1-07` は exact `SL-A1-02` save-load report に widened した report-local retained-artifact catalog と hit/miss query trace を export する retention-query widening に留まる。これは durable retained-artifact service / remote retrieval / expiry lifecycle を意味しない
- `product-alpha1/` と `practical-alpha1/` を混同しない
  - `product-alpha1/` は product/public alpha-1 line の source root
  - current repo state では `P-A1-26` の schema / CLI entrypoint fixture、`P-A1-27` の local same-session `run-local` / `session` / `attach` first cut、`P-A1-28` の local R0/R2 save first cut、`P-A1-29` の local/Docker transport と non-final viewer first cut、`P-A1-30` の native host launch bundle first cut、`P-A1-31` の CLI `demo` / release check / clean-clone docs を持つ
  - `demo` の object / avatar-preview package attach は accepted runtime execution ではなく deferred boundary evidence として読む

## current commands

```bash
python3 scripts/current_l2_guided_samples.py smoke-all --format json
python3 scripts/clean_near_end_samples.py smoke-all --format json
python3 scripts/sugoroku_world_samples.py check-all
python3 scripts/avatar_follow_samples.py check-all --format json
python3 scripts/typed_external_boundary_samples.py check-all --format json
python3 scripts/network_transport_samples.py check-all --format json
python3 scripts/projection_codegen_samples.py check-all --format json
python3 scripts/visual_debugger_viewer_samples.py check-all --format json
python3 scripts/mir_computational_samples.py check-all --format json
python3 scripts/posegraph_samples.py check-all --format json
python3 scripts/projection_boundary_samples.py check-all --format json
python3 scripts/engine_adapter_boundary_samples.py check-all --format json
python3 scripts/minimal_alpha1_patterns.py check-all --format json
cargo test -p mir-ast practical_alpha1_front_door -- --nocapture
python3 scripts/practical_alpha1_check.py check-all --format json
python3 scripts/practical_alpha1_run_local.py check-all --format json
python3 scripts/practical_alpha05_session.py check-all --format json
python3 scripts/practical_alpha08_session_hotplug.py check-all --format json
python3 scripts/practical_alpha09_devtools.py check-all --format json
python3 scripts/practical_alpha1_integrated_workflow.py check-all --format json
python3 scripts/practical_alpha1_attach.py check-all --format json
python3 scripts/practical_alpha1_transport.py check-all --format json
python3 scripts/practical_alpha1_export_devtools.py check-all --format json
python3 scripts/practical_alpha1_save_load.py check-all --format json
python3 scripts/practical_alpha1_avatar.py check-all --format json
python3 scripts/practical_alpha1_product_preview.py check-all --format json
cargo run -q -p mirrorea-cli -- check samples/product-alpha1/demo --format json
tmpdir=$(mktemp -d /tmp/mirrorea-alpha1-session-XXXXXX)
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/demo --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- attach 'session#product-alpha1-demo' samples/product-alpha1/demo/packages/debug-layer --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- save 'session#product-alpha1-demo' --savepoint 'savepoint#r0' --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- quiescent-save 'session#product-alpha1-demo' --savepoint 'savepoint#r2' --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- load 'savepoint#r0' --session 'session#product-alpha1-demo' --format json
bundle_dir=$(mktemp -d /tmp/mirrorea-alpha1-bundle-XXXXXX)
cargo run -q -p mirrorea-cli -- build-native-bundle samples/product-alpha1/demo --out "$bundle_dir" --format json
cargo run -q -p mirrorea-cli -- demo samples/product-alpha1/demo --out /tmp/mirrorea-alpha1-demo --format json
python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release
python3 scripts/operational_product_samples.py list --format json
python3 scripts/operational_product_samples.py check-all --format json
```

- `current_l2_guided_samples.py` は active current-L2 front-door compatibility wrapper であり、`list` / `smoke-all` / `closeout` を `clean_near_end_samples.py` へ forward する
- docs checks、storage guardrail、Cargo regression を含む broader validation floor は `samples_progress.md`、`progress.md`、`tasks.md` を参照する

## Full System V1 commands

Current Full System V1 commands:

```bash
python3 scripts/textual_mir_samples.py check-all --format json
python3 scripts/full_system_v1_samples.py check-all --format json
python3 scripts/full_system_v1_samples.py operational-matrix --format json
python3 scripts/full_system_v1_samples.py check-operational-all --format json
python3 scripts/posegraph_runtime_samples.py check-all --format json
python3 scripts/projection_v1_samples.py check-all --format json
python3 scripts/provider_admission_samples.py check-all --format json
python3 scripts/renderer_pose_backend_samples.py check-all --format json
python3 scripts/full_system_v1_release_check.py --format json check-all --out /tmp/mirrorea-full-v1-release
cargo test -p mir-semantics --test typed_ir_interpreter -- --nocapture
cargo test -p mir-runtime --test full_system_v1_session -- --nocapture
cargo test -p mir-runtime --test posegraph_runtime -- --nocapture
cargo test -p mir-runtime --test projection_ir -- --nocapture
cargo test -p mir-runtime --test provider_admission -- --nocapture
cargo test -p mir-runtime --test renderer_pose_backend -- --nocapture
cargo test -p mirrorea-cli --test full_system_v1_cli -- --nocapture
```

`scripts/full_system_v1_release_check.py` is the bounded line-level closeout command. It reruns the validation floor, focused Cargo tests, source-first helpers, Product Alpha compatibility anchors, representative Full V1 CLI surfaces, and writes per-command JSON reports plus static `bundle.json` / `index.html` viewer outputs.

## Surface Mir commands

Current `P-SURF-01` adds parser evidence and `P-SURF-02` adds indexed-state
semantic checker evidence. The following commands are runnable for `SURF-*` and
`IDX-*` rows:

```bash
python3 scripts/surface_mir_samples.py matrix --format json
python3 scripts/surface_mir_samples.py check-all --format json
python3 scripts/surface_mir_authoring_check.py check-all --format json
python3 scripts/surface_mir_release_check.py --format json plan --out /tmp/mirrorea-surface-release
python3 scripts/surface_mir_release_check.py --format json check-all --out /tmp/mirrorea-surface-release
cargo test -p mir-ast --test surface_mir_parser -- --nocapture
cargo test -p mir-semantics --test indexed_state_semantics -- --nocapture
```

Semantics / runtime / CLI commands remain future anchors:

```bash
cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture
cargo test -p mir-runtime --test source_patch_hotplug -- --nocapture
```

## move policy

- active sample は silent delete せず、archive へ移す
- generated artifact と source sample を混ぜない
- heavy disposable generated artifact は repo root ではなく external workdir を優先する
