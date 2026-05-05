# progress

最終更新: 2026-05-05 17:48 JST

## この文書について

- この文書は repo 全体の **operational workflow snapshot** です。
- 規範判断の正本は `specs/`、長期参照は `plan/`、実行証跡は `docs/reports/`、runnable sample dashboard は `samples_progress.md` です。
- 進捗率は primary metric ではありません。`100%` は外部開発者がその layer を実際に使える operational workflow または product/public layer だけに使います。
- helper / sidecar / report / expected JSON / first-floor runner は completion ではなく evidence として分類します。
- 古い package 履歴の詳細は `docs/reports/` と `plan/90-source-traceability.md` を参照し、この snapshot では current checkpoint / next gate / validation floor を優先します。

## project axis

```text
正しい理論に基づき、
正しく hot-plug でき、
Place をまたいで実行・通信・検証・可視化できる
仮想空間システムを作る。
```

この軸は Mir / Mirrorea / PrismCascade / Typed-Effect Wiring Platform の separability を消すものではありません。

## current position

- latest closeout package:
  `P-A1-31` clean-clone product alpha-1 validation / release candidate closeout
- current promoted reopen point:
  post-P-A1-31 maintenance or a new explicit final-public gate
- current reading:
  repo は **theory / first-floor carriers / evidence closeout** に加えて **bounded operational α-0.5 local observable runtime**、**bounded operational α-0.8 same-session hot-plug runtime**、**bounded operational α-0.9 session-bound devtools export**、**bounded practical α-1 integrated workflow carrier**、および **product alpha release-candidate workflow** を得た。`P-A1-25` で product/public-ready alpha-1 の境界、alpha `U1` defaults、`P-A1-26..31` package line を固定し、`P-A1-26` で `mirrorea-alpha check` と versioned product package schema first cut、`P-A1-27` で local same-session `run-local` / `session` / `attach`、`P-A1-28` で bounded message recovery rows、R0 local `save` / `load`、R2 local `quiescent-save` first cut、`P-A1-29` で local loopback TCP / Docker Compose TCP `transport`、non-final `export-devtools`、`view --check`、`P-A1-30` で native host launch `build-native-bundle` first cut、`P-A1-31` で source-backed attach matrix verified `demo` / release check / clean-clone docs を追加した。`--skip-docker` は partial local probe であり release-candidate readiness ではない。ただし final public viewer / telemetry ABI、durable audit、WAN/federation、distributed durable save/load は未完成
- self-driven status:
  bounded practical workflow までは自走済み。user prompt により alpha `U1` defaults は採用済みなので、`P-A1-26..31` は stop line を守る限り自走可能。final public grammar / ABI / WAN / distributed durable save-load は still user/final decision gate

## workflow-readiness axes

| 軸 | Workflow reading | Current status |
|---|---|---|
| 論理仕様 | boundary-fixed, not workflow completion by itself | `specs/18..25` で practical / operational / product alpha boundary を分けた。final public grammar / ABI は未固定 |
| ユーザ向け仕様 | reproducible workflow guidance exists for product alpha release candidate | README / Documentation / progress / tasks / samples dashboard に加え、`docs/hands_on/product_alpha1_01.md` と `docs/research_abstract/product_alpha1_01.md` で clean-clone product alpha guide を追加した |
| 実装 / 運用 | product alpha release-candidate workflow-ready | α-0.5 local session workflow、α-0.8 same-session hot-plug workflow、α-0.9 session-bound devtools workflow、product alpha `check/run-local/session/attach/save/load/quiescent-save/transport/export-devtools/view/build-native-bundle/demo/release-check` は再現可能。final-public grammar / ABI / WAN / distributed durable save-load は別 gate |

## line snapshot

| Line | Category | Workflow status | Current status | Next gap |
|---|---|---|---|---|
| current-L2 active floor | runnable evidence | evidence-backed runnable floor | `samples/clean-near-end/`、Sugoroku、Lean foundations / generated stubs、helper stack は runnable | final public parser/API は未固定 |
| Spaces alpha-0 | evidence line | evidence-closed only | Stage A..F は current-scope evidence であり、operational workflow completion ではない | operational α-0.5 / α-0.8 / α-0.9 とは別 |
| practical alpha-1 first floors | first-floor evidence | evidence-closed only | `RUN-01..04`、`HP-A1-01..07`、`TR-A1-01..07`、`VIS-A1-01..07`、`SL-A1-01..03`、`AV-A1-01..03`、`PE2E-01..09` は first-floor evidence | product/public-ready α-1 とは別 |
| practical alpha-1 integrated workflow | bounded workflow line | developer-reproducible bounded workflow | `P-A1-23` で `scripts/practical_alpha1_integrated_workflow.py`、`PA1W-01..08` を追加し、front-door / checker / same-session runtime / host-I/O / hot-plug / save-load / devtools / preview evidence を 1 workflow に束ねた | final public parser / viewer / telemetry ABI、product packaging は later |
| product alpha-1 release candidate | product/public alpha line | workflow-ready alpha release candidate, not final public product | `P-A1-25..31` で `mirrorea-alpha` command family、versioned package, same-session runtime, hot-plug, local/Docker transport, non-final devtools/viewer, local R0/R2 save/load, native host launch bundle, `demo`, release check, clean-clone docs を接続した | final public grammar / ABI / WAN / distributed durable save-load |
| operational α-0.5 | operational line | workflow-ready: local session workflow | `P-A1-20` で local session carrier + typed `AddOne` host-I/O lane を接続し、local observable runtime workflow を再現可能にした | broader host family は later |
| operational α-0.8 | operational line | workflow-ready: same-session hot-plug workflow | `P-A1-21` で debug/auth/rate-limit/object preview/deferred detach の same-session accepted/rejected/deferred/activation cut/observer-safe mutation を再現可能にした | accepted detach execution / distributed ordering は later |
| operational α-0.9 | operational line | workflow-ready: session-bound devtools workflow | `P-A1-22` で session-bound event DAG / route trace / membership timeline / witness relation / hot-plug lifecycle / fallback degradation / save-load timeline / redacted view / retention trace を再現可能にした | final public viewer / telemetry ABI、durable audit は later |
| final public product | final-public | not workflow-ready | product alpha boundary は fixed だが、final public grammar / ABI は別 gate | final user/final-public decisions |

## subsystem status

- **Mir core**
  finite decidable index fragment、effect row、lifetime/fallback、order/handoff、model-check second line、proof side export boundaryは current-L2 で整理済み
- **Mirrorea runtime / package line**
  `TermSignature`、`LayerSignature`、`MessageEnvelope`、`AuthEvidence`、`MembershipRegistry`、`PlaceCatalog`、`HotPlugRequest` / `HotPlugVerdict`、practical hot-plug / transport / save-load carriers、`practical_alpha05_session` session carrier はある
- **Typed external boundary**
  synthetic preview / canary に加えて、`AddOne` を bounded α-0.5 session carrier 上で direct semantic execution する minimal lane は actualize した。broader host family は later
- **Observability / devtools**
  export-side first floors に加え、α-0.9 で same session から event DAG / local route trace / membership timeline / witness relation / hot-plug lifecycle / fallback degradation / save-load timeline / observer-safe redacted view / retention-on-demand trace を引く non-final viewer/export lane が入った。final public telemetry service や durable audit backend は later
- **PrismCascade / Reversed Library**
  separable kept-later line。今回の operational α theory freeze の実装対象ではない

## macro phase map

| Macro | 主眼 | 現在位置 | 重さ | 自走可否 |
|---|---|---|---|---|
| `Macro 0` | repository memory / docs / traceability | 維持中 | light | 着手可能 |
| `Macro 1` | semantic kernel / invariant / boundary stabilization | current-L2 側は強い。operational α line は theory freeze 完了 | medium | 着手可能 |
| `Macro 5` | theorem / model-check / external verifier bridge | obligation export boundary は fixed、広い discharge は後段 | medium | 着手可能 |
| `Macro 6` | distributed fabric / shared-space / runtime evolution boundary | bounded α-0.8 same-session hot-plug runtime まで到達。accepted detach execution / distributed ordering は後段 | heavy | 着手可能 |
| `Macro 7` | toolchain / developer surface / public operational interface | α-0.5 / α-0.8 / α-0.9 operational line、bounded practical α-1 workflow、product alpha release-candidate workflow は完了。next は maintenance または final-public gate scoping | heavy | 着手可能 |
| `Macro 8` | domain / application realization | product alpha demo line が alpha defaults の下で開いた | heavy | 着手可能 |

## feature maturity rows

| Feature | Workflow status | 読み | 着手可否 |
|---|---|---|---|
| multi-node / fabric | evidence only | helper-local transport と alpha-0 evidence はあるが same-session operational shared-space は未完成 | 後段依存 |
| robustness via contracts / theorem / model-check boundary | boundary-fixed | static checker / model-check / proof side の stratification は fixed。外部 proof discharge は evidence expansion | 着手可能 |
| dynamic attach / detach / DAG-safe evolution | bounded workflow-ready for same-session attach | attach-time first-floor evidence と bounded same-session lifecycle はあるが accepted detach execution / migration / distributed ordering は未完成 | 着手可能 |
| `atomic_cut` と ordering / memory-order family | semantics fixed, evidence-backed | place-local rollback frontier と consistent-cut boundary は fixed、distributed durable family は later | 着手可能 |
| executable sample corpus | runnable evidence + bounded workflows | current-L2、practical alpha-1 first floors、bounded operational α-0.5 / α-0.8 / α-0.9 line、bounded practical α-1 integrated workflow は runnable。product alpha demo root は CLI check、local same-session run/session/attach、R0 save/load、bounded R2 quiescent-save、local/Docker transport、non-final devtools/viewer、native host launch bundle、`demo`、release check が runnable | 着手可能 |

## current blockers

- product alpha-1 release-candidate workflow は入ったが、final public grammar / ABI / WAN / distributed durable save-load / production packaging は未固定
- final public viewer / telemetry ABI、admin/full debug view、durable audit backend は未固定
- distributed durable save/load、stale witness / stale lease non-resurrection、queue/channel persistence は current promoted reopen point の外側
- final public grammar / ABI / WAN / distributed durable save-load / engine adapter scope は user/final decision gate のまま

## validation floor

- docs / hierarchy:
  `python3 -m unittest scripts.tests.test_validate_docs`
  `python3 scripts/check_source_hierarchy.py`
  `python3 scripts/validate_docs.py`
- product alpha CLI / schema:
  `cargo test -p mir-ast --test product_alpha1_package_schema -- --nocapture`
  `cargo test -p mir-runtime --test product_alpha1_session -- --nocapture`
  `cargo test -p mirrorea-cli --test alpha_cli -- --nocapture`
  `cargo run -q -p mirrorea-cli -- check samples/product-alpha1/demo --format json`
  `MIRROREA_ALPHA_SESSION_DIR=$(mktemp -d) cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/demo --format json`
  `MIRROREA_ALPHA_SESSION_DIR=<same-dir> cargo run -q -p mirrorea-cli -- save 'session#product-alpha1-demo' --savepoint 'savepoint#r0' --format json`
  `MIRROREA_ALPHA_SESSION_DIR=<same-dir> cargo run -q -p mirrorea-cli -- quiescent-save 'session#product-alpha1-demo' --savepoint 'savepoint#r2' --format json`
  `MIRROREA_ALPHA_SESSION_DIR=<same-dir> cargo run -q -p mirrorea-cli -- load 'savepoint#r0' --session 'session#product-alpha1-demo' --format json`
  `MIRROREA_ALPHA_SESSION_DIR=<same-dir> cargo run -q -p mirrorea-cli -- transport 'session#product-alpha1-demo' --mode local --format json`
  `MIRROREA_ALPHA_SESSION_DIR=<same-dir> cargo run -q -p mirrorea-cli -- transport 'session#product-alpha1-demo' --mode docker --format json`
  `MIRROREA_ALPHA_SESSION_DIR=<same-dir> cargo run -q -p mirrorea-cli -- export-devtools 'session#product-alpha1-demo' --out /tmp/mirrorea-alpha1-devtools --format json`
  `cargo run -q -p mirrorea-cli -- view /tmp/mirrorea-alpha1-devtools --check --format json`
  `cargo run -q -p mirrorea-cli -- build-native-bundle samples/product-alpha1/demo --out /tmp/mirrorea-alpha1-bundle --format json`
  `cargo run -q -p mirrorea-cli -- demo samples/product-alpha1/demo --out /tmp/mirrorea-alpha1-demo --format json`
  `python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release`
  `cargo test -p mir-runtime --test product_alpha1_transport_devtools -- --nocapture`
- formatting / diff:
  `cargo fmt --check`
  `git diff --check`
- `cargo test -p mir-runtime --test practical_alpha1_local_runtime -- --nocapture`
- `cargo test -p mir-runtime --test practical_alpha05_host_io -- --nocapture`
- `cargo test -p mir-runtime --test practical_alpha05_session -- --nocapture`
- `cargo test -p mir-runtime --test practical_alpha08_session_hotplug -- --nocapture`
- `cargo test -p mir-runtime --test practical_alpha09_devtools -- --nocapture`
- `python3 scripts/practical_alpha1_run_local.py check-all --format json`
- `python3 scripts/practical_alpha1_export_devtools.py check-all --format json`
- `python3 scripts/practical_alpha1_product_preview.py check-all --format json`
- `python3 scripts/practical_alpha1_integrated_workflow.py check-all --format json`
- `python3 scripts/practical_alpha05_session.py check-all --format json`
- `python3 scripts/practical_alpha08_session_hotplug.py check-all --format json`
- `python3 scripts/practical_alpha09_devtools.py check-all --format json`
- `python3 -m unittest scripts.tests.test_practical_alpha1_run_local scripts.tests.test_practical_alpha05_session`
- `python3 -m unittest scripts.tests.test_practical_alpha08_session_hotplug scripts.tests.test_practical_alpha09_devtools`
- `python3 -m unittest scripts.tests.test_practical_alpha1_integrated_workflow`

## recent log

- 2026-05-05 17:14 JST
- 2026-05-05 17:48 JST
  `P-A1-31` review hardening として、`--skip-docker` を partial non-release probe に降格し、demo attach matrix verification、source-backed admin membership/capability authority、canonical admin session store reopen evidence、observer-safe session artifact、concrete viewer panel rendering、release-check validation floor / JSON semantic checks を追加した。
- 2026-05-05 17:14 JST
  `P-A1-31` で `mirrorea-alpha demo`、`scripts/product_alpha1_release_check.py check-all`、product alpha hands-on / research docs を追加し、debug/auth/rate-limit layer attach、deferred object/avatar-preview attach、local/Docker transport、viewer、save/load/quiescent-save、native host bundle を release-candidate workflow として束ねた。final public product / grammar / ABI / WAN / distributed durable save-load ではない。
- 2026-05-05 16:35 JST
  `P-A1-30` で `mirrorea-alpha build-native-bundle` を追加し、compiled CLI、versioned package bundle、observer-safe devtools assets、manifest、launch metadata、run script、verification/provenance reports を含む native host launch bundle first cut を生成した。`NativeExecutionPolicy = Disabled`、package-native execution 非 claim、signature-is-safety 非 claim、direct Mir-to-machine-code 非 goal は CLI tests と probe で確認。`demo` command / release validation はまだ後段。
- 2026-05-05 15:53 JST
  `P-A1-29` で product session carrier に local loopback TCP `transport --mode local`、Docker Compose TCP world/participant `transport --mode docker`、non-final `export-devtools` JSON/HTML bundle、`view --check` を追加した。observer-safe panel set、admin/debug `kept_later`、redaction leak guard、Docker endpoint reports、lane separation は focused Rust tests と CLI probe で確認。この package close 時点では native launch bundle、release validation、final public viewer / telemetry ABI、WAN/federation、R3/R4 durable distributed save/load は未実装だった。
- 2026-05-05 15:06 JST
  `P-A1-28` で product session carrier に DAG-linked bounded `MessageState` / `TransportContract` / `RecoveryPolicy` rows、R0 local `save` / `load`、R2 local `quiescent-save` を追加した。`NoInFlight` / session-carried `AllPlacesSealed` / `NoPostCutSend`、load-admissibility reject、duplicate event-ID guard、observer-safe mutation payload は runtime/CLI tests で確認。R3/R4 durable distributed save/load、WAN/federation、product transport/viewer、native bundle、release validation はまだ後段。
- 2026-05-05 14:48 JST
  `P-A1-27` で `crates/mir-runtime::product_alpha1_session`、`mirrorea-alpha run-local` / `session` / `attach`、CLI local session store を追加し、reviewer 指摘後に declared host-I/O input、activation cut、auth/capability/membership/witness gate、observer-safe policy monotonicity、hash付き atomic session store を harden した。transport / save-load / quiescent-save / viewer / native bundle / release validation はまだ後段。
- 2026-05-05 14:00 JST
  `P-A1-26` で `mirrorea-cli` crate と `mirrorea-alpha` binary、product alpha-1 `package.mir.json` schema loader/checker、`samples/product-alpha1/demo/` fixture root を追加した。`check` は explicit accepted evidence を返し、`run-local` / `session` / `attach` / `transport` / `save` / `load` / `quiescent-save` / `export-devtools` / `view` / `build-native-bundle` / `demo` は structured unsupported diagnostic を返す。product alpha-1 workflow-ready claim はまだしない。
- 2026-05-05 13:14 JST
  `P-A1-25` で product/public-ready alpha-1 の境界を `specs/25` / `plan/50` に固定し、alpha `U1` defaults、`P-A1-26..31` package line、CLI / package schema / product demo / transport / message recovery / quiescent-save / viewer / native launch bundle / release validation の stop line を snapshot docs と required documentation scaffold に同期した。product alpha-1 implementation はまだ未完。
- 2026-05-05 12:32 JST
  root tracked Markdown を audit し、`README.md` の practical alpha-1 evidence inventory を canonical docs / dashboard 参照へ圧縮した。`AGENTS.md` と `tasks.md` の stale percentage wording を workflow readiness / evidence classification へ同期し、α-0.5 / α-0.8 / α-0.9 / bounded practical α-1 workflow の concrete behavior validation を再実行した。
- 2026-05-05 11:59 JST
  `P-A1-24` で workflow-readiness policy を反映し、進捗率ではなく external developer reproducible workflow を snapshot の primary metric とした。helper / sidecar / report / expected JSON / first-floor runner は completion ではなく evidence として分類する方針へ同期した。
- 2026-05-05 11:33 JST
  `P-A1-23` で `scripts/practical_alpha1_integrated_workflow.py`、`scripts/tests/test_practical_alpha1_integrated_workflow.py`、`PA1W-01..08` を追加し、source front-door / checker / same-session runtime / typed host-I/O / hot-plug / save-load / session devtools / product-preview evidence を bounded practical α-1 workflow として束ねた。`VIS-A1-01` の expected devtools bundle も runtime-side positive guard reason refs に同期した。
- 2026-05-05 11:17 JST
  `P-A1-22` で `crates/mir-runtime::practical_alpha09_devtools`、example `mir_practical_alpha05_session -- export-devtools`、`scripts/practical_alpha09_devtools.py`、`OA09-01..09` を actualizeし、α-0.5 / α-0.8 session carrier 上の event DAG / local route trace / membership timeline / witness relation / hot-plug lifecycle / fallback degradation / save-load timeline / observer-safe redacted view / retention-on-demand trace を bounded α-0.9 session-bound devtools export に接続した。
- 2026-05-05 10:18 JST
  `P-A1-21` で `crates/mir-runtime::practical_alpha08_hotplug_session`、example `mir_practical_alpha05_session -- attach`、`scripts/practical_alpha08_session_hotplug.py`、`OA08-01..10` を actualizeし、debug / auth / rate-limit / object preview / deferred detach を α-0.5 session carrier 上で same-session accepted/rejected/deferred と observer-safe lifecycle summary に接続した。
- 2026-05-05 09:47 JST
  `P-A1-20` で `crates/mir-runtime::practical_alpha05_host_io`、example `mir_practical_alpha05_session -- host-io`、`samples/practical-alpha1/packages/oa05-07-add-one-host-io`、`OA05-07` を actualizeし、typed external `AddOne` direct execution lane を α-0.5 same-session carrier と observer-safe export に接続した。
- 2026-05-05 09:26 JST
  `P-A1-19` で `RUN-03/04` capability / witness negative rows、`practical_alpha05_session` same-session carrier、session-bound observe/save/load loop、event DAG / observer-safe session export を actualizeし、operational α-0.5 の残 gap を typed host-I/O direct execution lane に絞った。
- 2026-05-05 08:32 JST
  `P-A1-18` で operational α-0.5 / α-0.8 / α-0.9 の completion condition、verification stratification、cut/save-load semantics、auth layer algebra、typed observability、typed host boundary を `specs/19..24` と `plan/45..49` に固定し、snapshot docs を evidence / first-floor / operational の読み分けへ同期した。
