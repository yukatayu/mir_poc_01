# progress

最終更新: 2026-05-07 11:03 JST

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
  `P-OPS-22` portal/shard starter revisit
- current promoted reopen point:
  broader Sugoroku revisit
- current reading:
  repo は **theory / first-floor carriers / evidence closeout** に加えて **bounded operational α-0.5 local observable runtime**、**bounded operational α-0.8 same-session hot-plug runtime**、**bounded operational α-0.9 session-bound devtools export**、**bounded practical α-1 integrated workflow carrier**、**product alpha release-candidate workflow**、さらにその次段の **canonical operational product sample suite** を得た。`P-A1-25..31` で `mirrorea-alpha` product line と `samples/product-alpha1/demo/` release-candidate workflow を固定した上で、`P-OPS-01` は `samples/product-alpha1/operational/`、`specs/26..27`、`plan/51..52`、`scripts/operational_product_samples.py` を追加し、`WorldCore -> MembershipChat -> SugorokuWorld` package/import chain、explicit debug/auth/rate-limit attach packages、local/Docker transport、observer-safe devtools export、R0/R2 save/load、native host launch bundle、projection intent、portal/shard future inventory を 1 つの canonical suite として接続した。`P-OPS-03` はその上で `MembershipChat` に bounded direct host boundary を actualize し、`P-OPS-13` で current lane を bounded room-oriented `ChatText("hello room") -> "room#lobby message accepted: hello room"` へ widen した。`run-local` と session-bound devtools export から同じ observer-safe room-chat lane を確認できるが、final room-chat service や stdio builtin は主張しない。`P-OPS-04` は `SugorokuWorld` に bounded same-session roll / publish / witness / handoff / stale membership reject scenario を actualize し、`run-local` / session / devtools export / helper closeout から同じ runtime evidence を確認できるようにした。`P-OPS-05` は `projection.profile.json` を schema-backed target / packet / FFI inventory として `check` accepted obligation、runtime plan summary、observer-safe devtools projection panel、helper `release-check` / `check-all` に接続した。`P-OPS-06` は `portal-worldlink/` root を追加し、bounded same-session discrete handoff evidence を `run-local`、observer-safe devtools export、helper closeout に接続しつつ `future/portal-worldlink/` blueprint は保持した。`P-OPS-07` は `two-shard-hard-boundary/` root を追加し、bounded same-session offer / prepare / commit / old-owner reject / missing-witness reject / stale-config reject evidence を `run-local`、observer-safe devtools export、helper closeout に接続しつつ `future/two-shard-hard-boundary/` と `spatial-shard-future.profile.json` は retained blueprint inventory として保持した。`P-OPS-09` はその suite を外部開発者が再利用しやすいように、`templates/world-core-starter/` と authoring guide を追加し、template-only starter を active operational roots と混同しない bounded `author -> check -> run-local -> session -> export-devtools -> view --check` 入口を固定した。`P-OPS-08` はその次段として backend feasibility inventory を docs-first に actualize し、current actualized backend-adjacent path が `native host launch bundle` だけであること、WASM client host と direct LLVM/native projection backend は inventory-only であること、packet/FFI/projection と auth/membership/capability/witness lane preservation が future reopen prerequisite であることを固定した。`P-OPS-10` はさらに `templates/membership-chat-starter/` と `templates/sugoroku-world-starter/` を追加し、validated starter catalog を `WorldCore -> MembershipChat -> SugorokuWorld` mainstream chain まで widen した上で、dependency retarget obligation を authoring docs / dashboard に同期した。`P-OPS-11` は `future/gradient-observation.profile.json` と corresponding guide を追加し、observer-only shard overlap reading、freshness fields、replication non-default reading、fallback behavior を `planned_only` inventory として固定した。`P-OPS-12` は portal/shard starter boundary を docs-first に固定し、starter catalog を `SugorokuWorld` で止め、portal/shard authoring は active roots を study/copy boundary にし、`future/` files は non-executable inventory のまま保つ current decision を同期した。`P-OPS-15` はその次段として `two-shard-gradient-observation/` separate root を actualize し、observer-only gradient view / handoff hint / write reject / stale-view drop / missing-freshness reject evidence を `run-local`、observer-safe devtools export、helper closeout に接続しつつ `future/gradient-observation.profile.json` 自体は non-executable inventory のまま保持した。`P-OPS-16` はその上で final-public queue を整理し、next promoted line を public packaging adoption target scoping に絞った。`P-OPS-17` は続けて `scripts/product_alpha1_installed_binary_check.py check-all`、built `target/debug/mirrorea-alpha` probe、native host launch bundle `run.sh check/view` probe、product alpha hands-on / research / roadmap / dashboard sync を追加し、current first public-ish adoption candidate を installed binary + native host launch bundle として actualize した。`P-OPS-18` はさらに `specs/25` と installed-binary helper output を使い、current hardening target を versioned `package.mir.json`、documented `mirrorea-alpha` command family、native host launch bundle replay surface に絞った。`P-OPS-19` はその narrowed front door の current shipped surface を helper / bundle stdout / bundle manifest / verification report に machine-readable `shipped_surface` block として actualize し、built-binary `check` / `build-native-bundle` / `demo`、bundle replay `run.sh check` / `run.sh view`、bundled CLI / package root / `manifest.json` / `launch.json` / `run.sh` / `README.md` / observer-safe supporting artifacts を current alpha replay bundle surface に固定した。`P-OPS-20` はさらに helper に machine-readable `distribution_scope` を追加し、broader public distribution は current line では未定義、すなわち developer-built `mirrorea-alpha` binary + locally generated native host launch bundle 以外の archive / installer / system-package / auto-update / hosted-service shape をまだ持たないと固定した。`P-OPS-21` はその operational side queue-shaping として helper に machine-readable `room_chat_scope` を追加し、current `membership-chat` lane は bounded single-message room-oriented `ChatText` だけであり、multi-message room surface、transport-coupled chat lane、room-history service、stdio builtin は未定義のままと固定した。`P-OPS-22` はその次段として helper に machine-readable `portal_shard_starter_scope` を追加し、validated starter catalog は `templates/sugoroku-world-starter` で止まり、portal/shard authoring は active executable roots を study/copy boundary に使い、`future/` inventory は non-executable のまま保つ current line を machine-readable に固定した。next promoted line は broader Sugoroku revisit であり、portal/shard starter reopening は non-promoted later package に戻った。final textual grammar、final Rust library ABI、final viewer/devtools bundle ABI、WAN/federation、distributed durable save-load は still later gate のままである
- self-driven status:
  bounded practical workflow、operational product suite gradient runtime first cut、portal/shard starter boundary、bounded room-chat widening、maintenance / dashboard freshness、final-public gate scoping、installed-binary adoption probe first cut、final grammar / ABI scoping、shipped-surface hardening、broader public distribution narrowing、broader room-chat revisit、portal/shard starter revisit までは自走済み。user prompt により alpha `U1` defaults と operational suite line は採用済みなので、next self-driven reopen point は broader Sugoroku revisit である。WAN / distributed durable save-load は still later gate

## workflow-readiness axes

| 軸 | Workflow reading | Current status |
|---|---|---|
| 論理仕様 | boundary-fixed, not workflow completion by itself | `specs/18..25` で practical / operational / product alpha boundary を分けた。final public grammar / ABI は未固定 |
| ユーザ向け仕様 | reproducible workflow guidance exists for product alpha release candidate, operational suite, bounded external authoring starter catalog, and backend comparison inventory | README / Documentation / progress / tasks / samples dashboard に加え、`docs/hands_on/product_alpha1_01.md`、`docs/research_abstract/product_alpha1_01.md`、`docs/hands_on/operational_product_sample_01.md`、`docs/research_abstract/operational_product_sample_01.md`、`docs/hands_on/operational_package_authoring_01.md`、`docs/research_abstract/operational_package_authoring_01.md`、`docs/hands_on/operational_backend_inventory_01.md`、`docs/research_abstract/operational_backend_inventory_01.md` で release-candidate root、operational suite root、template-only authoring starter catalog、backend inventory の使い分けを追加した |
| 実装 / 運用 | product alpha release-candidate workflow-ready plus widened canonical operational suite | α-0.5 local session workflow、α-0.8 same-session hot-plug workflow、α-0.9 session-bound devtools workflow、product alpha `check/run-local/session/attach/save/load/quiescent-save/transport/export-devtools/view/build-native-bundle/demo/release-check` と installed-binary adoption probe、さらに operational suite `WorldCore -> MembershipChat -> SugorokuWorld -> PortalWorldLink -> TwoShardHardBoundary -> TwoShardGradientObservation` の `check/run-local/session/attach/save/quiescent-save/transport/export-devtools/view/build-native-bundle/check-all` は再現可能で、`membership-chat` は bounded room-oriented `ChatText` lane、`sugoroku-world` は bounded roll / publish / witness / handoff / stale membership reject scenario、`portal-worldlink` は bounded same-session discrete handoff scenario、`two-shard-hard-boundary` は bounded same-session hard-authority scenario、`two-shard-gradient-observation` は bounded same-session observer-only gradient scenario を持つ。final-public grammar / ABI / WAN / distributed durable save-load は別 gate |

## line snapshot

| Line | Category | Workflow status | Current status | Next gap |
|---|---|---|---|---|
| current-L2 active floor | runnable evidence | evidence-backed runnable floor | `samples/clean-near-end/`、Sugoroku、Lean foundations / generated stubs、helper stack は runnable | final public parser/API は未固定 |
| Spaces alpha-0 | evidence line | evidence-closed only | Stage A..F は current-scope evidence であり、operational workflow completion ではない | operational α-0.5 / α-0.8 / α-0.9 とは別 |
| practical alpha-1 first floors | first-floor evidence | evidence-closed only | `RUN-01..04`、`HP-A1-01..07`、`TR-A1-01..07`、`VIS-A1-01..07`、`SL-A1-01..03`、`AV-A1-01..03`、`PE2E-01..09` は first-floor evidence | product/public-ready α-1 とは別 |
| practical alpha-1 integrated workflow | bounded workflow line | developer-reproducible bounded workflow | `P-A1-23` で `scripts/practical_alpha1_integrated_workflow.py`、`PA1W-01..08` を追加し、front-door / checker / same-session runtime / host-I/O / hot-plug / save-load / devtools / preview evidence を 1 workflow に束ねた | final public parser / viewer / telemetry ABI、product packaging は later |
| product alpha-1 release candidate | product/public alpha line | workflow-ready alpha release candidate, not final public product | `P-A1-25..31` で `mirrorea-alpha` command family、versioned package, same-session runtime, hot-plug, local/Docker transport, non-final devtools/viewer, local R0/R2 save/load, native host launch bundle, `demo`, release check, clean-clone docs を接続し、`P-OPS-17` で built `mirrorea-alpha` binary と generated bundle `run.sh` を direct probe する installed-binary adoption helper を追加し、`P-OPS-18` で current hardening target を versioned `package.mir.json`、documented CLI surface、native host launch bundle replay に絞り、`P-OPS-19` で helper / bundle stdout / manifest / verification report に machine-readable `shipped_surface` block を追加して alpha replay bundle surface と evidence-only artifacts を分け、`P-OPS-20` で helper に machine-readable `distribution_scope` を追加して developer-built binary + generated host launch bundle 以外の broader distribution shape を未定義に固定した | later user/final broader distribution decision |
| operational product sample suite | operational product line | workflow-ready canonical suite, not final public product | `P-OPS-01` で `samples/product-alpha1/operational/`、`specs/26..27`、`plan/51..52`、`scripts/operational_product_samples.py` を追加し、`WorldCore -> MembershipChat -> SugorokuWorld` chain、shared attach packages、projection intent、portal/shard future inventory を bounded operational replay として同期した。`P-OPS-03` で `membership-chat` に direct host boundary、`P-OPS-13` で bounded room-oriented `ChatText` lane、`P-OPS-04` で `sugoroku-world` に bounded roll / publish / witness / handoff / stale membership reject runtime evidence と corresponding devtools evidence、`P-OPS-05` で schema-backed projection target / packet / FFI inventory summary を `check` / runtime plan / devtools / helper に追加し、`P-OPS-06` で `portal-worldlink` bounded same-session discrete handoff root、`P-OPS-07` で `two-shard-hard-boundary` bounded same-session hard-authority root、`P-OPS-15` で separate `two-shard-gradient-observation` bounded observer-only gradient root と corresponding devtools / helper evidence を actualize した。`P-OPS-09` で `templates/world-core-starter/` と bounded authoring guide を追加し、`P-OPS-08` で `native host launch bundle` / WASM / LLVM comparison inventory と future feasibility requirements を docs-first に追加し、`P-OPS-10` で `templates/membership-chat-starter/` / `templates/sugoroku-world-starter/` と dependency-retarget guidance を追加し、`P-OPS-11` で `gradient-observation.profile.json` と corresponding guide を追加し、`P-OPS-12` で portal/shard starter boundary を docs-first に固定し、`P-OPS-14` で queue / validator / roadmap / dashboard wording を current state に同期し、`P-OPS-17..20` で product-side installed-binary adoption / grammar-scope / shipped-surface / distribution-scope reading を guide / roadmap / dashboard へ同期し、`P-OPS-21` で `membership-chat` / helper closeout に machine-readable `room_chat_scope` を追加して bounded single-message room-oriented `ChatText` lane を current floor として固定し、`P-OPS-22` で suite `check-all` に machine-readable `portal_shard_starter_scope` を追加して validated starter catalog が `templates/sugoroku-world-starter` で止まり portal/shard authoring は active executable roots を使う current boundary を固定した | broader Sugoroku revisit |
| operational α-0.5 | operational line | workflow-ready: local session workflow | `P-A1-20` で local session carrier + typed `AddOne` host-I/O lane を接続し、local observable runtime workflow を再現可能にした | broader host family は later |
| operational α-0.8 | operational line | workflow-ready: same-session hot-plug workflow | `P-A1-21` で debug/auth/rate-limit/object preview/deferred detach の same-session accepted/rejected/deferred/activation cut/observer-safe mutation を再現可能にした | accepted detach execution / distributed ordering は later |
| operational α-0.9 | operational line | workflow-ready: session-bound devtools workflow | `P-A1-22` で session-bound event DAG / route trace / membership timeline / witness relation / hot-plug lifecycle / fallback degradation / save-load timeline / redacted view / retention trace を再現可能にした | final public viewer / telemetry ABI、durable audit は later |
| final public product | final-public | not workflow-ready | product alpha boundary は fixed だが、final public grammar / ABI は別 gate | final user/final-public decisions |

## subsystem status

- **Mir core**
  finite decidable index fragment、effect row、lifetime/fallback、order/handoff、model-check second line、proof side export boundaryは current-L2 で整理済み
- **Mirrorea runtime / package line**
  `TermSignature`、`LayerSignature`、`MessageEnvelope`、`AuthEvidence`、`MembershipRegistry`、`PlaceCatalog`、`HotPlugRequest` / `HotPlugVerdict`、practical hot-plug / transport / save-load carriers、`practical_alpha05_session` session carrier に加えて、product alpha package kinds `world_core` / `membership_chat` / `sugoroku_world` / `portal_worldlink` / `two_shard_hard_boundary` / `two_shard_gradient_observation`、`membership-chat` bounded room-oriented `ChatText` host boundary、`sugoroku-world` bounded roll/publish/witness/handoff/stale-reject scenario、`portal-worldlink` bounded discrete handoff scenario、`two-shard-hard-boundary` bounded hard-authority scenario、`two-shard-gradient-observation` bounded observer-only gradient scenario、schema-backed projection inventory summary、observer-safe operational devtools inventory、dependency-preserving native host launch bundle copy line、`samples/product-alpha1/operational/` suite、template-only `world-core` / `membership-chat` / `sugoroku-world` starter catalog、そして docs-first backend comparison inventory がある
- **Typed external boundary**
  synthetic preview / canary に加えて、`AddOne` を bounded α-0.5 session carrier 上で direct semantic execution する minimal lane と、operational `membership-chat` root 上の bounded room-oriented `ChatText` lane は actualize した。broader multi-message host family は later
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
| `Macro 7` | toolchain / developer surface / public operational interface | α-0.5 / α-0.8 / α-0.9 operational line、bounded practical α-1 workflow、product alpha release-candidate workflow、operational product suite gradient runtime first cut、template-only authoring starter catalog、backend comparison inventory、portal/shard starter boundary、bounded room-chat widening、maintenance / dashboard freshness、final-public gate scoping、installed-binary adoption probe first cut、final grammar / ABI scoping、shipped-surface hardening、broader public distribution narrowing、broader room-chat revisit、portal/shard starter revisitまで到達。next は broader Sugoroku revisit | heavy | 着手可能 |
| `Macro 8` | domain / application realization | product alpha demo root に加えて `WorldCore -> MembershipChat -> SugorokuWorld -> PortalWorldLink -> TwoShardHardBoundary -> TwoShardGradientObservation` operational suite root が開いた | heavy | 着手可能 |

## feature maturity rows

| Feature | Workflow status | 読み | 着手可否 |
|---|---|---|---|
| multi-node / fabric | bounded local/Docker alpha workflow for product scope | product alpha demo と operational Sugoroku root は same-session carrier と controlled local/Docker TCP transport を持ち、projection inventory は schema-backed summary として `check` / runtime / devtools に現れる。ただし production multi-node / WAN federation / distributed durable replay ではない | 着手可能 |
| robustness via contracts / theorem / model-check boundary | boundary-fixed | static checker / model-check / proof side の stratification は fixed。外部 proof discharge は evidence expansion | 着手可能 |
| dynamic attach / detach / DAG-safe evolution | bounded workflow-ready for same-session attach | attach-time first-floor evidence と bounded same-session lifecycle はあるが accepted detach execution / migration / distributed ordering は未完成 | 着手可能 |
| `atomic_cut` と ordering / memory-order family | semantics fixed, evidence-backed | place-local rollback frontier と consistent-cut boundary は fixed、distributed durable family は later | 着手可能 |
| executable sample corpus | runnable evidence + bounded workflows | current-L2、practical alpha-1 first floors、bounded operational α-0.5 / α-0.8 / α-0.9 line、bounded practical α-1 integrated workflow は runnable。product alpha demo root は CLI check、local same-session run/session/attach、R0 save/load、bounded R2 quiescent-save、local/Docker transport、non-final devtools/viewer、native host launch bundle、`demo`、release check が runnable。`samples/product-alpha1/operational/` は `world-core` / `membership-chat` / `sugoroku-world` / `portal-worldlink` / `two-shard-hard-boundary` / `two-shard-gradient-observation` suite、`membership-chat` bounded room-oriented `ChatText` lane、`sugoroku-world` bounded runtime scenario、schema-backed projection inventory、`portal-worldlink` bounded discrete handoff scenario、`two-shard-hard-boundary` bounded hard-authority scenario、`two-shard-gradient-observation` bounded observer-only gradient scenario、template-only `world-core` / `membership-chat` / `sugoroku-world` starter catalog、helper `check-all`、retained shard blueprints を持つ | 着手可能 |

## current blockers

- product alpha-1 release-candidate workflow と operational suite first cut に加えて installed-binary adoption probe、grammar / ABI scoping、shipped-surface hardening、distribution-scope narrowing は入った。broader installed/public distribution は current line では未定義のまま later user/final decision に戻し、WAN / distributed durable save-load はその後段に残る
- operational suite では broader room-chat revisit と portal/shard starter revisit が current boundary hardening として閉じ、broader Sugoroku revisit が next queue になった
- final public viewer / telemetry ABI、admin/full debug view、durable audit backend は未固定
- distributed durable save/load、stale witness / stale lease non-resurrection、queue/channel persistence は current promoted reopen point の外側
- WAN / distributed durable save-load / engine adapter scope は broader room-chat revisit 後も later gate のまま

## validation floor

- docs / hierarchy:
  `python3 -m unittest scripts.tests.test_validate_docs`
  `python3 scripts/check_source_hierarchy.py`
  `python3 scripts/validate_docs.py`
- product alpha CLI / schema:
  `python3 -m unittest scripts.tests.test_product_alpha1_installed_binary_check`
  `python3 -m unittest scripts.tests.test_product_alpha1_release_check`
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
  `python3 scripts/product_alpha1_installed_binary_check.py --format json check-all --out /tmp/mirrorea-alpha1-installed-binary-check`
  `cargo test -p mir-runtime --test product_alpha1_transport_devtools -- --nocapture`
- operational product suite:
  `python3 -m unittest scripts.tests.test_operational_product_samples`
  `cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/world-core --format json`
  `cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/membership-chat --format json`
  `cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/sugoroku-world --format json`
  `cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/portal-worldlink --format json`
  `cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/two-shard-hard-boundary --format json`
  `cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/two-shard-gradient-observation --format json`
  `MIRROREA_ALPHA_SESSION_DIR=$(mktemp -d) cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/operational/membership-chat --format json`
  `MIRROREA_ALPHA_SESSION_DIR=<same-dir> cargo run -q -p mirrorea-cli -- export-devtools 'session#operational-membership-chat' --out /tmp/mirrorea-ops-chat-viewer --format json`
  `cargo run -q -p mirrorea-cli -- view /tmp/mirrorea-ops-chat-viewer --check --format json`
  `MIRROREA_ALPHA_SESSION_DIR=$(mktemp -d) cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/operational/portal-worldlink --format json`
  `MIRROREA_ALPHA_SESSION_DIR=<same-dir> cargo run -q -p mirrorea-cli -- export-devtools 'session#operational-portal-worldlink' --out /tmp/mirrorea-ops-portal-viewer --format json`
  `cargo run -q -p mirrorea-cli -- view /tmp/mirrorea-ops-portal-viewer --check --format json`
  `python3 scripts/operational_product_samples.py run-portal-worldlink --format json`
  `MIRROREA_ALPHA_SESSION_DIR=$(mktemp -d) cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/operational/two-shard-hard-boundary --format json`
  `MIRROREA_ALPHA_SESSION_DIR=<same-dir> cargo run -q -p mirrorea-cli -- export-devtools 'session#operational-two-shard-hard-boundary' --out /tmp/mirrorea-ops-shard-viewer --format json`
  `cargo run -q -p mirrorea-cli -- view /tmp/mirrorea-ops-shard-viewer --check --format json`
  `python3 scripts/operational_product_samples.py run-two-shard-hard-boundary --format json`
  `MIRROREA_ALPHA_SESSION_DIR=$(mktemp -d) cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/operational/two-shard-gradient-observation --format json`
  `MIRROREA_ALPHA_SESSION_DIR=<same-dir> cargo run -q -p mirrorea-cli -- export-devtools 'session#operational-two-shard-gradient-observation' --out /tmp/mirrorea-ops-gradient-viewer --format json`
  `cargo run -q -p mirrorea-cli -- view /tmp/mirrorea-ops-gradient-viewer --check --format json`
  `python3 scripts/operational_product_samples.py run-two-shard-gradient-observation --format json`
  `MIRROREA_ALPHA_SESSION_DIR=$(mktemp -d) cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/operational/sugoroku-world --format json`
  `python3 scripts/operational_product_samples.py run-sugoroku --format json`
  `MIRROREA_ALPHA_SESSION_DIR=<same-dir> cargo run -q -p mirrorea-cli -- attach 'session#operational-sugoroku' samples/product-alpha1/operational/packages/debug-layer --format json`
  `MIRROREA_ALPHA_SESSION_DIR=<same-dir> cargo run -q -p mirrorea-cli -- save 'session#operational-sugoroku' --savepoint 'savepoint#ops-r0' --format json`
  `MIRROREA_ALPHA_SESSION_DIR=<same-dir> cargo run -q -p mirrorea-cli -- quiescent-save 'session#operational-sugoroku' --savepoint 'savepoint#ops-r2' --format json`
  `MIRROREA_ALPHA_SESSION_DIR=<same-dir> cargo run -q -p mirrorea-cli -- transport 'session#operational-sugoroku' --mode local --format json`
  `MIRROREA_ALPHA_SESSION_DIR=<same-dir> cargo run -q -p mirrorea-cli -- transport 'session#operational-sugoroku' --mode docker --format json`
  `MIRROREA_ALPHA_SESSION_DIR=<same-dir> cargo run -q -p mirrorea-cli -- export-devtools 'session#operational-sugoroku' --out /tmp/mirrorea-ops-viewer --format json`
  `python3 scripts/operational_product_samples.py export-devtools --format json`
  `cargo run -q -p mirrorea-cli -- view /tmp/mirrorea-ops-viewer --check --format json`
  `cargo run -q -p mirrorea-cli -- build-native-bundle samples/product-alpha1/operational/sugoroku-world --out /tmp/mirrorea-ops-bundle --format json`
  `python3 scripts/operational_product_samples.py check-all --format json`
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

- 2026-05-07 11:03 JST
  `P-OPS-22` で `scripts/operational_product_samples.py` に machine-readable `portal_shard_starter_scope` を追加し、validated starter catalog が `templates/sugoroku-world-starter` で止まり portal/shard authoring は active executable roots を study/copy boundary に使う current lineを固定した。starter duplicates は未定義のままとし、next reopen point を broader Sugoroku revisit に進めた。
- 2026-05-07 10:42 JST
  `P-OPS-21` で `scripts/operational_product_samples.py` に machine-readable `room_chat_scope` を追加し、current `membership-chat` lane を bounded single-message room-oriented `ChatText` に固定した。multi-message / transport-coupled / room-history / stdio は current line では未定義のままとし、next reopen point を portal/shard starter revisit に進めた。
- 2026-05-07 10:22 JST
  `P-OPS-20` で helper に machine-readable `distribution_scope` を追加し、current delivery unit を developer-built `mirrorea-alpha` binary + locally generated native host launch bundle のみに固定した。archive / installer / system-package / auto-update / hosted-service shape は current line では未定義とし、next reopen point は broader room-chat revisit に進めた。
- 2026-05-07 10:09 JST
  `P-OPS-19` で helper / bundle stdout / bundle manifest / verification report に machine-readable `shipped_surface` block を追加し、built-binary `check` / `build-native-bundle` / `demo` と bundle replay `run.sh check` / `run.sh view`、bundled CLI / package root / `manifest.json` / `launch.json` / `run.sh` / `README.md` / observer-safe supporting artifacts を current alpha replay bundle surface として固定した。next reopen point は broader public distribution narrowing。
- 2026-05-07 09:28 JST
  `P-OPS-17` で `scripts/product_alpha1_installed_binary_check.py`、built `mirrorea-alpha` probe、bundle `run.sh check/view` probe、product alpha guide / summary / roadmap / dashboard sync を追加し、current first public-ish adoption candidate を installed binary + native host launch bundle として固定した。next reopen point は final grammar / ABI scoping。
- 2026-05-07 09:12 JST
  `P-OPS-16` で final-public gate scoping を閉じ、next promoted line を public packaging adoption target scoping に絞った。current actualized `mirrorea-alpha` / native host launch bundle / local-Docker host path を first public-ish candidate とし、grammar / ABI / WAN / durability は後段 gate に戻した。
- 2026-05-07 08:28 JST
  `P-OPS-15` で `two-shard-gradient-observation/` separate root を追加し、bounded observer-only gradient view / handoff hint / write reject / stale-view drop / missing-freshness reject evidence を `run-local` / observer-safe devtools export / helper closeout に接続した。next reopen point は final-public gate scoping。
- 2026-05-07 08:28 JST
  `P-OPS-14` で maintenance / dashboard freshness を閉じ、queue / validator / roadmap / dashboard wording を current state に同期した。next reopen point は gradient observation runtime first cut。
- 2026-05-07 01:45 JST
  `P-OPS-13` で `MembershipChat` root と starter catalog を bounded room-oriented `ChatText("hello room") -> "room#lobby message accepted: hello room"` lane に widen し、schema/runtime/helper/guide/dashboard を同期した。next reopen point は maintenance / dashboard freshness。
- 2026-05-07 01:32 JST
  `P-OPS-12` で portal/shard starter boundary を docs-first に固定し、starter catalog を `SugorokuWorld` で止めて active roots / `future/` inventory split を保つ current decision を同期した。next reopen point は broader room-chat lane widening。
- 2026-05-07 01:10 JST
  `P-OPS-11` で `future/gradient-observation.profile.json` と guide を追加し、observer-only gradient widening を `planned_only` inventory として固定した。next reopen point は portal/shard starter decision。
- 2026-05-07 00:56 JST
  `P-OPS-10` で `templates/membership-chat-starter/` と `templates/sugoroku-world-starter/` を追加し、dependency-retarget guidance を authoring docs / dashboard に同期した。next reopen point は gradient observation profile。
- 2026-05-07 00:39 JST
  `P-OPS-08` で backend feasibility inventory を docs-first に追加し、current actualized path が `native host launch bundle` のみであることと WASM/LLVM inventory-only boundary を固定した。next reopen point は broader operational template catalog。
- 2026-05-07 00:25 JST
  `P-OPS-09` で `templates/world-core-starter/` と bounded operational package authoring guide を追加し、template-only starter の `check` / `run-local` / `session` / `export-devtools` / `view --check` 入口を固定した。next reopen point は `P-OPS-08`。
- 2026-05-07 00:00 JST
  `P-OPS-07` で `two-shard-hard-boundary/` root を追加し、bounded same-session offer / prepare / commit / old-owner reject / missing-witness reject / stale-config reject evidence を `run-local` / observer-safe devtools export / helper closeout に接続した。next reopen point は `P-OPS-09`。
- 2026-05-06 23:32 JST
  `P-OPS-06` で `portal-worldlink/` root を追加し、bounded same-session discrete handoff evidence を `run-local` / observer-safe devtools export / helper closeout に接続した。next reopen point は `P-OPS-07`。
- 2026-05-06 23:01 JST
  `P-OPS-05` で `projection.profile.json` を schema-backed inventory に formalize し、`check` / runtime plan / devtools projection panel / helper semantic checks を同期した。next reopen point は `P-OPS-06`。
- 2026-05-06 22:42 JST
  `P-OPS-04` で `SugorokuWorld` root に bounded same-session roll / publish / witness / handoff / stale membership reject scenario を追加し、runtime/devtools/helper semantic checks、hands-on / research docs、dashboard を同期した。next reopen point は `P-OPS-05`。
- 2026-05-06 22:27 JST
  `P-OPS-03` で `MembershipChat` root に bounded `EchoText("Taro") -> "Hello, Taro!"` direct host boundary を追加し、schema/runtime validation、observer-safe session export、session-bound devtools export、helper `run-membership-chat` / `check-all` semantic checks、hands-on / research docs を同期した。next reopen point は `P-OPS-04`。
- 2026-05-06 21:12 JST
  `P-OPS-01` closeout hardening として、operational Docker transport を専用 compose file に切り替え、native bundle に shared attach packages と attach reports を含め、helper `release-check` / `check-all` に deferred object/avatar attach rows を組み込み、route/config devtools payload を bounded summary / kept-later wording に寄せた。operational suite は `accepted` で再検証済み。
- 2026-05-05 19:41 JST
  post-`P-A1-31` final docs audit で、`specs/25` success wording、`plan/50` current-state wording、`progress.md` multi-node/fabric row、`tasks.md` distributed durability reopen wording、source hierarchy product handoff presence を product alpha release-candidate 実態へ同期した。reports は audit 対象外。
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
