# progress

最終更新: 2026-05-05 14:48 JST

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
  `P-A1-27` product demo same-session runtime
- current promoted reopen point:
  `P-A1-28` message failure/recovery + quiescent-save
- current reading:
  repo は **theory / first-floor carriers / evidence closeout** に加えて **bounded operational α-0.5 local observable runtime**、**bounded operational α-0.8 same-session hot-plug runtime**、**bounded operational α-0.9 session-bound devtools export**、および **bounded practical α-1 integrated workflow carrier** を得た。`P-A1-25` で product/public-ready alpha-1 の境界、alpha `U1` defaults、`P-A1-26..31` package line を固定し、`P-A1-26` で `mirrorea-alpha check` と versioned product package schema first cut を追加した。`P-A1-27` で product demo root は `mirrorea-alpha run-local` / `session` / `attach` を通じて local file-backed same-session carrier、runtime plan、core fabric envelope validation、typed host-I/O observation、debug-layer hot-plug lifecycle を持つようになった。ただし local/Docker transport command behavior、message failure/recovery execution、quiescent-save、product viewer、native launch bundle、release validation、final public viewer / telemetry ABI、durable audit、distributed durable save/load は未完成
- self-driven status:
  bounded practical workflow までは自走済み。user prompt により alpha `U1` defaults は採用済みなので、`P-A1-26..31` は stop line を守る限り自走可能。final public grammar / ABI / WAN / distributed durable save-load は still user/final decision gate

## workflow-readiness axes

| 軸 | Workflow reading | Current status |
|---|---|---|
| 論理仕様 | boundary-fixed, not workflow completion by itself | `specs/18..25` で practical / operational / product alpha boundary を分けた。final public grammar / ABI は未固定 |
| ユーザ向け仕様 | reproducible workflow guidance exists for bounded α lines | README / Documentation / progress / tasks / samples dashboard で evidence / first-floor / operational / bounded workflow / product alpha boundary の読み分けを更新した。clean-clone product alpha guide は `P-A1-31` まで未完成 |
| 実装 / 運用 | workflow-ready only where session workflow is reproducible | α-0.5 local session workflow、α-0.8 same-session hot-plug workflow、α-0.9 session-bound devtools workflow は再現可能。product alpha CLI は `check` と local same-session `run-local` / `session` / `attach` first cut まで実装済みだが、transport / save-load / viewer / native bundle / release validation は `P-A1-28..31` の未実装範囲 |

## line snapshot

| Line | Category | Workflow status | Current status | Next gap |
|---|---|---|---|---|
| current-L2 active floor | runnable evidence | evidence-backed runnable floor | `samples/clean-near-end/`、Sugoroku、Lean foundations / generated stubs、helper stack は runnable | final public parser/API は未固定 |
| Spaces alpha-0 | evidence line | evidence-closed only | Stage A..F は current-scope evidence であり、operational workflow completion ではない | operational α-0.5 / α-0.8 / α-0.9 とは別 |
| practical alpha-1 first floors | first-floor evidence | evidence-closed only | `RUN-01..04`、`HP-A1-01..07`、`TR-A1-01..07`、`VIS-A1-01..07`、`SL-A1-01..03`、`AV-A1-01..03`、`PE2E-01..09` は first-floor evidence | product/public-ready α-1 とは別 |
| practical alpha-1 integrated workflow | bounded workflow line | developer-reproducible bounded workflow | `P-A1-23` で `scripts/practical_alpha1_integrated_workflow.py`、`PA1W-01..08` を追加し、front-door / checker / same-session runtime / host-I/O / hot-plug / save-load / devtools / preview evidence を 1 workflow に束ねた | final public parser / viewer / telemetry ABI、product packaging は later |
| product alpha-1 local same-session runtime | product/public alpha line | local same-session first-cut, not release-ready | `P-A1-25` で boundary を固定し、`P-A1-26` で `mirrorea-alpha check` と schemaを追加し、`P-A1-27` で `run-local` / `session` / `attach` を local session store 上の product carrier に接続した | `P-A1-28..31` recovery / quiescent-save / viewer / bundle / release validation |
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
| `Macro 7` | toolchain / developer surface / public operational interface | α-0.5 / α-0.8 / α-0.9 operational line、bounded practical α-1 workflow、product alpha boundary recut、alpha CLI/schema first cut、product local same-session runtime first cut は完了。next は recovery/quiescent-save | heavy | 着手可能 |
| `Macro 8` | domain / application realization | product alpha demo line が alpha defaults の下で開いた | heavy | 着手可能 |

## feature maturity rows

| Feature | Workflow status | 読み | 着手可否 |
|---|---|---|---|
| multi-node / fabric | evidence only | helper-local transport と alpha-0 evidence はあるが same-session operational shared-space は未完成 | 後段依存 |
| robustness via contracts / theorem / model-check boundary | boundary-fixed | static checker / model-check / proof side の stratification は fixed。外部 proof discharge は evidence expansion | 着手可能 |
| dynamic attach / detach / DAG-safe evolution | bounded workflow-ready for same-session attach | attach-time first-floor evidence と bounded same-session lifecycle はあるが accepted detach execution / migration / distributed ordering は未完成 | 着手可能 |
| `atomic_cut` と ordering / memory-order family | semantics fixed, evidence-backed | place-local rollback frontier と consistent-cut boundary は fixed、distributed durable family は later | 着手可能 |
| executable sample corpus | runnable evidence + bounded workflows | current-L2、practical alpha-1 first floors、bounded operational α-0.5 / α-0.8 / α-0.9 line、bounded practical α-1 integrated workflow は runnable。product alpha demo root は CLI check と local same-session run/session/attach first cut が runnable | 着手可能 |

## current blockers

- product alpha-1 CLI / versioned package schema / demo root / local same-session runtime first cut は入ったが、local/Docker transport product command behavior、message failure/recovery、bounded R2 quiescent-save、product viewer、native launch bundle、clean-clone validation は未実装
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
