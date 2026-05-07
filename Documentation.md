# ドキュメント要約

## この文書の役割

この文書は、repo 全体の **current snapshot を短く正確に読む入口** です。

- 規範判断の正本は `specs/`
- 長期の repository memory は `plan/`
- workflow / evidence snapshot は `progress.md`
- current task map は `tasks.md`
- runnable sample dashboard は `samples_progress.md`
- 実行証跡は `docs/reports/`

## まず repo をどう読むべきか

この repo は、Mir / Mirrorea / PrismCascade / Typed-Effect Wiring Platform を **意図的に separable** に保った研究用 workspace です。主眼は Mir current-L2 と、その上に積む Mirrorea shared-space / hot-plug / observability / host-boundary line にあります。

読み分けで重要なのは、次の 6 つを混同しないことです。

- **repo-local alpha-ready current layer**
  `samples/clean-near-end/`、helper、Lean foundation、report まで含めて動かせる current-L2 の実行足場
- **current-scope evidence**
  `samples/alpha/` と `specs/13..17` / `plan/39..43` による alpha-0 helper-local / runtime-private closeout 群
- **practical alpha-1 first-floor evidence**
  `samples/practical-alpha1/` と `specs/18` / `plan/44` による front-door / checker / runtime / hot-plug / transport / devtools / save-load / preview の first-floor toolchain
- **operational alpha readiness**
  `specs/19..24` / `plan/45..49` で定義した α-0.5 / α-0.8 / α-0.9 の same-session operational 条件
- **product/public-ready Mirrorea Spaces alpha-1**
  `specs/25` / `plan/50` で定義した alpha-stable CLI、versioned package format、same-session product demo、quiescent save、viewer、native launch bundle、clean-clone validation の product alpha line。final textual `.mir` grammar、WAN/federation、distributed durable save/load、arbitrary native execution、final viewer / telemetry service は non-goal
- **operational product sample suite**
  `specs/26` / `plan/51` で定義した `WorldCore -> MembershipChat -> SugorokuWorld -> PortalWorldLink -> TwoShardHardBoundary -> TwoShardGradientObservation` suite、shared attach packages、deployment/projection inventory、retained portal/shard blueprint inventory。current line では `MembershipChat` の bounded room-oriented `ChatText` host boundary、`SugorokuWorld` の bounded roll / publish / witness / handoff / stale membership reject scenario、`projection.profile.json` の schema-backed target / packet / FFI inventory、`PortalWorldLink` の bounded same-session discrete handoff cut、`TwoShardHardBoundary` の bounded same-session two-shard offer / prepare / commit / old-owner reject / missing-witness reject / stale-config reject cut、そして `TwoShardGradientObservation` の bounded observer-only gradient view / handoff hint / write reject / stale-view drop / missing-freshness reject cut が `check`、runtime plan、observer-safe devtools / helper closeout に actualize 済み。`future/portal-worldlink/` と `future/two-shard-hard-boundary/` blueprint は残す
- **final public product**
  final parser grammar、public checker/runtime/verifier API、public adapter / viewer / projection / hot-plug / transport surface、packaging、external contract まで含む最終形

現在 repo は repo-local alpha-ready current layer、current-scope evidence、practical alpha-1 first-floor evidence に加えて、bounded operational α-0.5 / α-0.8 / α-0.9 と bounded practical α-1 integrated workflow carrier まで actualize 済みです。`P-A1-25` で product/public-ready alpha-1 の境界と alpha defaults は固定し、`P-A1-26` で `mirrorea-alpha check` と versioned product package schema first cut を追加しました。`P-A1-27` では `mirrorea-alpha run-local` / `session` / `attach` と product same-session carrier first cut を追加し、`P-A1-28` では bounded message recovery rows、R0 local `save` / `load`、R2 local `quiescent-save` を同じ session file に接続しました。`P-A1-29` では同じ session carrier に local loopback TCP / Docker Compose TCP `transport`、non-final `export-devtools` JSON/HTML bundle、`view --check` を接続しました。`P-A1-30` では `build-native-bundle` が compiled Rust CLI、versioned package bundle、observer-safe devtools assets、manifest、run script、verification/provenance reports を持つ native host launch bundle を生成します。`P-A1-31` では `mirrorea-alpha demo`、`scripts/product_alpha1_release_check.py check-all`、clean-clone hands-on guide / research summary を追加し、product alpha release-candidate workflow を再現可能にしました。`P-OPS-17` では `scripts/product_alpha1_installed_binary_check.py check-all` を追加し、built `target/debug/mirrorea-alpha` binary、generated native host launch bundle、bundle `run.sh check` / `run.sh view` を current first public-ish adoption probe として再検証できるようにしました。`P-OPS-18` ではさらに `specs/25` と同 helper output を使い、current hardening target を versioned `package.mir.json`、documented `mirrorea-alpha` command family、native host launch bundle replay surface に絞りました。`P-OPS-19` ではその narrowed front door の current shipped surface を machine-readable `shipped_surface` block として helper / bundle report / manifest / verification report に通し、built-binary `check` / `build-native-bundle` / `demo` と bundle replay `run.sh check` / `run.sh view`、bundled CLI / package root / `manifest.json` / `launch.json` / `run.sh` / `README.md` / observer-safe supporting artifacts を current alpha replay bundle surface に限定しました。other bundled reports、admin/debug session-store artifacts、final viewer/devtools ABI は compatibility promise の外に残ります。`P-OPS-20` はさらに machine-readable `distribution_scope` を helper に追加し、broader public distribution は current line では未定義、すなわち archive / installer / system package / auto-update / hosted-service shape をまだ持たず、current delivery unit は developer-built `mirrorea-alpha` binary + locally generated native host launch bundle のみであると固定しました。これは alpha-stable front-door / shipped-surface / broader-distribution queue narrowing であり、final public CLI/API/ABI、final textual grammar、final Rust library ABI、final viewer/devtools bundle ABI ではありません。`P-OPS-01` では `samples/product-alpha1/operational/`、`specs/26`、`plan/51`、`scripts/operational_product_samples.py` を追加し、`WorldCore -> MembershipChat -> SugorokuWorld` の canonical operational suite と projection/portal/shard future inventory を固定しました。`P-OPS-03` では `MembershipChat` に bounded direct text host boundary を actualize し、`P-OPS-13` ではそれを bounded room-oriented `ChatText("hello room") -> "room#lobby message accepted: hello room"` lane へ widen しました。`P-OPS-21` ではさらに helper-reported `room_chat_scope` を追加し、current lane は bounded single-message room-oriented `ChatText` に留まり、multi-message / transport-coupled / room-history / stdio shapes は未定義と固定しました。`P-OPS-22` では続けて helper-reported `portal_shard_starter_scope` を追加し、validated starter catalog が `templates/sugoroku-world-starter` で止まり、portal/shard authoring は active executable roots を study/copy boundary に使い、`future/` inventory は非 executable のまま保つ current decision を machine-readable に固定しました。`P-OPS-23` ではさらに helper-reported `sugoroku_scope` を追加し、current `SugorokuWorld` carrier は bounded deterministic same-session roll / publish / witness / handoff / stale-membership reject scenario に留まり、interactive turn choice surface、broader negative-row catalog、networked multi-participant control は未定義と machine-readable に固定しました。`P-OPS-25` ではさらに helper-reported `widening_queue_scope` を更新し、room-chat reopening、portal/shard starter reopening、broader Sugoroku reopening は current line では non-promoted、later user-final distribution decision が next promoted comparison であることを machine-readable に固定しました。`P-OPS-26` ではさらに helper-reported `user_final_decision_scope` を追加し、current delivery unit は developer-built binary + generated host launch bundle、current catalog scope は bounded product alpha-1 narrow showcase、broader final distribution / final shared-space catalog breadth は user-spec-required gate であり、current self-driven operational reopenings は exhausted であることを machine-readable に固定しました。same-session runtime と observer-safe devtools export から同じ lane / carrier を確認できますが、final room-chat service、interactive multiplayer game completion、stdio builtin、portal/shard starter catalog、broader final distribution / final catalog breadth は主張しません。`P-OPS-04` では `SugorokuWorld` に bounded same-session roll / publish / witness / handoff / stale membership reject scenario を actualize し、same-session runtime、observer-safe devtools export、helper closeout から同じ runtime evidence を確認できるようにしました。`P-OPS-05` では `deployments/projection/projection.profile.json` を `ops-product-projection-v0` schema として validation し、`check` の accepted obligation、runtime plan の `projection_inventory` summary、observer-safe devtools projection panel、helper `release-check` / `check-all` の semantic check まで接続しました。`P-OPS-06` では `portal-worldlink/` root を追加し、bounded same-session discrete handoff evidence を `run-local`、observer-safe devtools export、helper closeout に接続しつつ `future/portal-worldlink/` blueprint は保持しました。`P-OPS-07` では `two-shard-hard-boundary/` root を追加し、bounded same-session two-shard offer / prepare / commit / old-owner reject / missing-handoff-witness reject / stale-config reject evidence を `run-local`、observer-safe devtools export、helper closeout に接続しつつ `future/two-shard-hard-boundary/` と `spatial-shard-future.profile.json` は retained blueprint inventory として保持しました。`P-OPS-09` では `samples/product-alpha1/operational/templates/world-core-starter/` と `docs/hands_on/operational_package_authoring_01.md` / `docs/research_abstract/operational_package_authoring_01.md` を追加し、template-only starter を active operational roots と分離したまま external developer 向け `author -> check -> run-local -> session -> export-devtools -> view --check` の bounded authoring入口を固定しました。`P-OPS-08` では `docs/hands_on/operational_backend_inventory_01.md` と `docs/research_abstract/operational_backend_inventory_01.md` を追加し、current backend-adjacent actualization が `native host launch bundle` だけであること、WASM client host と direct LLVM/native projection backend は docs-first inventory に留まること、そして packet/FFI/projection/auth-capability-witness lanes を bypass しない future feasibility requirements を明示しました。`P-OPS-10` では `templates/membership-chat-starter/` と `templates/sugoroku-world-starter/` を追加し、validated starter catalog を `world_core` から `membership_chat` / `sugoroku_world` まで広げた上で、dependency retarget guidance を authoring docs / dashboard に同期しました。`P-OPS-11` では `gradient-observation.profile.json` と reader-facing guide を追加し、portal/shard future line における observer-only gradient widening を `planned_only` profile として固定しました。`P-OPS-12` では portal/shard authoring boundary を docs-first に固定し、starter catalog を intentionally `SugorokuWorld` で止め、portal/shard authoring は active executable roots から始め、`future/` blueprints は non-executable inventory に留める current decision を guide / roadmap / dashboard へ同期しました。`P-OPS-15` では separate `two-shard-gradient-observation/` root を追加し、observer-only gradient view / handoff hint / write reject / stale-view drop / missing-freshness reject evidence を `check` / `run-local` / observer-safe devtools export / helper `release-check` / `check-all` へ actualize しつつ、`future/gradient-observation.profile.json` 自体は non-executable inventory のまま保持しました。ただし final public product、final public grammar / ABI、final public viewer / telemetry ABI、R3/R4 durable distributed save/load、WAN/federation、arbitrary native package execution、final server/client binary split、continuous spatial sync、direct LLVM backend は引き続き別 gate / non-goal です。

## current active floor

active canonical sample は `samples/clean-near-end/` です。base current-L2 corpus は `samples/current-l2/`、active Lean mechanization evidence は `samples/lean/` に置きます。

- `typing/`
  finite-index first strong typing layer
- `order-handoff/`
  publication / witness / handoff relation family
- `model-check/`
  Peterson / broken mutex による second-line verification
- `modal/`
  `stable` / `later` / `published(room)` / `witnessed(...)` の current mode line
- `sugoroku-world/`
  empty world server へ SugorokuGame を runtime attach する Mir / Mirrorea vertical slice

旧 active sample line は active path から外し、archive に退避しています。

## Mirrorea line の現在地

```text
OS/network substrate
  -> Mir core
  -> Typed external boundary
  -> Mirrorea fabric/runtime
  -> practical toolchain
  -> Spaces product
  -> Reversed Library
```

- **alpha-0 evidence line**
  `specs/13..17`、`plan/39..43`、`samples/alpha/` が current-scope evidence を担います。Stage A..F は evidence reference であり、operational α-0.5 / α-0.8 / α-0.9 completion ではありません。
- **practical alpha-1 first-floor / workflow line**
  `specs/18`、`plan/44`、`samples/practical-alpha1/` が first-floor toolchain を担います。`RUN-01..04`、`HP-A1-01..07`、`TR-A1-01..07`、`VIS-A1-01..07`、`SL-A1-01..03`、`AV-A1-01..03`、`PE2E-01..09` は current repo state で actualize 済みですが、いずれも first-floor evidence です。`P-A1-23` で `scripts/practical_alpha1_integrated_workflow.py` と `PA1W-01..08` を追加し、source front-door / checker / same-session runtime / host-I/O / hot-plug / save-load / session devtools / product-preview evidence を 1 つの bounded developer workflow として再現できるようにしました。ただし final public product-ready completion は意味しません。
- **operational alpha theory-freeze line**
  `specs/19..24`、`plan/45..49` が α-0.5 local observable runtime、α-0.8 same-session hot-plug runtime、α-0.9 session-bound devtools readiness の completion condition を固定します。2026-05-05 時点の latest operational closeout は `P-A1-22` で、`P-A1-18` の bounded theory freeze、`P-A1-19` の session carrier、`P-A1-20` の typed external `AddOne` lane、`P-A1-21` の same-session hot-plug lane に続けて、`crates/mir-runtime::practical_alpha09_devtools`、example `mir_practical_alpha05_session -- export-devtools`、`scripts/practical_alpha09_devtools.py`、`OA09-01..09` により bounded operational α-0.9 session-bound devtools export を actualize しました。

## いま何があり、何がまだ無いか

既にあるもの:

- practical checker / runtime / hot-plug / transport / devtools / save-load / preview の **distinct carrier split**
- event DAG export、observer-safe route trace、membership timeline export、fallback degradation export、redacted observer view、report-local retention query trace
- local-only save/load roundtrip と stale-membership non-resurrection first-floor rows
- attach-time auth / rate-limit / object preview / deferred detach の first-floor rows
- bounded α-0.5 session carrier 上の minimal typed external `AddOne` direct execution lane
- bounded α-0.8 same-session hot-plug runtime over the same session carrier
- bounded α-0.9 session-bound devtools export over the same carrier
- bounded practical α-1 integrated workflow carrier over the existing first-floor and operational evidence

まだ無いもの:

- final public viewer / telemetry ABI
- durable audit backend / remote retained-artifact retrieval
- distributed durable save/load
- final-public product hardening beyond the product alpha release-candidate workflow

## 重要な境界

- `Place` は participant と同一ではなく、state / queue / capability / visibility / observation frontier を持つ execution locus として読む
- standard I/O は Mir core primitive ではなく、typed external adapter boundary 側に残す
- auth / authorization / membership / capability / witness は transport に潰さない
- visualization / telemetry は label / authority / redaction / retention を持つ typed effect として扱う
- `atomic_cut` は place-local rollback frontier であり、durable/distributed commit ではない
- local save/load は distributed durable save/load と同一ではない

## どこを次に読むか

- live status / next reopen point:
  `progress.md`、`tasks.md`
- runnable sample dashboard:
  `samples_progress.md`
- practical alpha-1 first-floor:
  `specs/18-practical-alpha1-scope.md`、`plan/44-practical-alpha1-roadmap.md`
- operational alpha theory freeze:
  `specs/19-verification-stratification.md`
  `specs/20-cut-save-load-semantics.md`
  `specs/21-auth-layer-algebra.md`
  `specs/22-observability-devtools-semantics.md`
  `specs/23-typed-external-host-boundary.md`
  `specs/24-operational-alpha05-alpha08-readiness.md`
  `plan/45-operational-alpha05-roadmap.md`
  `plan/46-operational-alpha08-roadmap.md`
  `plan/47-operational-alpha09-devtools-roadmap.md`
  `plan/48-theory-freeze-proof-obligations.md`
  `plan/49-host-io-and-session-runtime-roadmap.md`
- product/public alpha-1 boundary:
  `specs/25-product-alpha1-public-boundary.md`
  `plan/50-product-alpha1-public-boundary-roadmap.md`
- operational product sample suite:
  `specs/26-operational-product-sample-suite.md`
  `specs/27-spatial-portal-and-shard-extension-boundary.md`
  `plan/51-operational-product-sample-roadmap.md`
  `plan/52-portal-spatial-world-roadmap.md`
- hands-on product alpha commands:
  `docs/hands_on/product_alpha1_01.md`
  `docs/hands_on/operational_product_sample_01.md`
  `docs/hands_on/operational_package_authoring_01.md`
  `docs/hands_on/operational_backend_inventory_01.md`
  `docs/hands_on/operational_portal_shard_starter_boundary_01.md`
- legacy hands-on closeout commands:
  `docs/hands_on/current_phase_closeout_01.md`

## snapshot の読み方

- `progress.md` と `samples_progress.md` は進捗率ではなく workflow status / evidence classification を primary metric にする
- helper / sidecar / report / expected JSON / first-floor runner は completion ではなく evidence として読む
- `100%` は外部開発者が実際に再現・使用できる operational workflow または product/public layer だけに使う
- `PA1W-*` は bounded practical workflow ready として読み、product/public-ready α-1 とは読まない
