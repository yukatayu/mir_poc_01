# tasks

最終更新: 2026-05-07 01:45 JST

## この文書について

- この文書は repo 全体の **current task map** です。
- 規範判断の正本は `specs/`、長期比較と source trace は `plan/`、runnable sample 状態は `samples_progress.md`、実行証跡は `docs/reports/` に置きます。
- append-only 履歴ではありません。current checkpoint、次に詰める gate、blocker を読める snapshot として保ちます。
- 進捗率は primary metric ではありません。helper / sidecar / report / expected JSON / first-floor runner は completion ではなく evidence として書きます。
- `100%` は外部開発者がその layer を実際に再現・使用できる operational workflow または product/public layer だけに使います。

## current task-level status

- active executable floor は維持されています:
  `samples/clean-near-end/`、Sugoroku world、avatar follow、typed external preview、network canary、projection/codegen bridge、viewer prototype inventory。`samples/current-l2/` は base source corpus、`samples/lean/` は Lean evidence / generated theorem stub corpus として分けて扱います。
- `P-A1-18` により operational α-0.5 / α-0.8 / α-0.9 の completion condition と theory boundary は fixed されました:
  `specs/19..24` と `plan/45..49` が verification stratification、`atomic_cut` / save-load semantics、auth layer algebra、typed observability、typed host boundary、operational readiness definition を担います。
- `P-A1-19` により bounded α-0.5 same-session carrier は actualize 済みです:
  `crates/mir-runtime::practical_alpha05_session`、example `mir_practical_alpha05_session`、`scripts/practical_alpha05_session.py`、`RUN-03/04` capability / witness negatives により、check -> runtime plan -> run-local -> observe -> save -> load を同一 carrier に束ねる実行面が入りました。
- `P-A1-20` により bounded operational α-0.5 line は actualize 済みです:
  `crates/mir-runtime::practical_alpha05_host_io`、example `mir_practical_alpha05_session -- host-io`、`samples/practical-alpha1/packages/oa05-07-add-one-host-io`、`OA05-07` により、typed external `AddOne` direct execution lane を同じ session carrier、event DAG、observer-safe export に接続しました。
- `P-A1-21` により bounded operational α-0.8 line は actualize 済みです:
  `crates/mir-runtime::practical_alpha08_hotplug_session`、example `mir_practical_alpha05_session -- attach`、`scripts/practical_alpha08_session_hotplug.py`、`OA08-01..10` により、debug / auth / rate-limit / object preview / unsupported-runtime fallback companion source / deferred detach boundary を α-0.5 session carrier 上で same-session accepted/rejected/deferred / activation cut / observer-safe lifecycle summary に接続しました。
- `P-A1-22` により bounded operational α-0.9 line は actualize 済みです:
  `crates/mir-runtime::practical_alpha09_devtools`、example `mir_practical_alpha05_session -- export-devtools`、`scripts/practical_alpha09_devtools.py`、`OA09-01..09` により、event DAG / local route trace / membership timeline / witness relation / hot-plug lifecycle / fallback degradation / save-load timeline / observer-safe redacted view / retention-on-demand trace を同じ session carrier から export できるようにしました。
- `P-A1-23` により bounded practical α-1 integrated workflow carrier は actualize 済みです:
  `scripts/practical_alpha1_integrated_workflow.py`、`scripts/tests/test_practical_alpha1_integrated_workflow.py`、`PA1W-01..08` により、existing first-floor front-door / checker / runtime / host-I/O / hot-plug / save-load / session devtools / product-preview evidence を 1 つの bounded developer workflow に束ねました。product/public-ready α-1 ではありません。
- `P-A1-25` により product/public-ready alpha-1 boundary は fixed されました:
  `specs/25-product-alpha1-public-boundary.md`、`plan/50-product-alpha1-public-boundary-roadmap.md` により、alpha `U1` defaults、canonical Rust CLI direction、versioned `package.mir.json`、same-session product demo、message recovery / quiescent-save、product viewer、native host launch bundle、release validation の package line を固定しました。product alpha-1 implementation completion、arbitrary native package execution、signature-is-safety ではありません。
- `P-A1-26` により product alpha CLI / schema first cut は actualize 済みです:
  `crates/mirrorea-cli`、binary `mirrorea-alpha`、`crates/mir-ast::product_alpha1`、`samples/product-alpha1/demo/` により、versioned product `package.mir.json` schema、`check` accepted evidence、direct `.mir` non-goal diagnostic、full alpha command family の structured unsupported diagnostic を追加しました。same-session product runtime や product-ready α-1 completion ではありません。
- `P-A1-27` により product demo same-session runtime first cut は actualize 済みです:
  `crates/mir-runtime::product_alpha1_session`、`mirrorea-alpha run-local` / `session` / `attach`、CLI local session store により、product demo は runtime plan、core fabric envelope validation、typed host-I/O observation、debug-layer attach lifecycle、membership/witness/route/save-load/recovery state carrier を同じ session file に保持します。local/Docker transport command behavior、message recovery execution、quiescent-save、viewer、native launch bundle、product-ready α-1 completion ではありません。
- `P-A1-28` により product message recovery / save-load first cut は actualize 済みです:
  `MessageState` / `TransportContract` / `RecoveryPolicy` rows、`mirrorea-alpha save` / `load` / `quiescent-save`、R0 local save/load、bounded R2 local quiescent-save を同じ product session carrier に接続しました。`NoInFlight` / `AllPlacesSealed` / `NoPostCutSend` positive と in-flight reject negative は runnable tests で確認します。R3/R4 durable distributed save/load、WAN/federation、product-ready α-1 completion ではありません。
- `P-A1-29` により product transport / viewer first cut は actualize 済みです:
  `mirrorea-alpha transport --mode local` は same-session loopback TCP round trip、`transport --mode docker` は controlled Docker Compose TCP world/participant round trip、`export-devtools` は product session 由来の non-final JSON/HTML bundle、`view --check` は bundle openability / panel presence check を提供します。final public viewer / telemetry ABI、WAN/federation、native launch bundle、release validation、product-ready α-1 completion ではありません。
- `P-A1-30` により product native launch bundle first cut は actualize 済みです:
  `mirrorea-alpha build-native-bundle` は compiled Rust CLI、versioned package bundle、observer-safe devtools assets、manifest、launch metadata、run script、verification/provenance reports を含む native host launch bundle を生成します。`NativeExecutionPolicy = Disabled`、package-native execution 非 claim、signature-is-safety 非 claim、direct Mir-to-machine-code 非 goal を明示します。
- `P-A1-31` により product alpha release-candidate workflow は actualize 済みです:
  `mirrorea-alpha demo` と `scripts/product_alpha1_release_check.py check-all` により product package front-door、checker、same-session runtime、typed host-I/O、source-backed debug/auth/rate-limit layer attach、deferred object/avatar-preview attach boundary、local/Docker transport、concrete non-final viewer、local save/load、bounded quiescent-save、native host launch bundleを束ねます。`--skip-docker` は partial local probe であり release-candidate ready ではありません。final public product / grammar / ABI / WAN / distributed durable save-load ではありません。
- `P-OPS-01` により canonical operational product sample suite first cut は actualize 済みです:
  `samples/product-alpha1/operational/`、`specs/26..27`、`plan/51..52`、`scripts/operational_product_samples.py` により、`WorldCore -> MembershipChat -> SugorokuWorld` package/import chain、explicit debug/auth/rate-limit attach packages、local/Docker transport、observer-safe devtools/view、R0/R2 save/load、native host launch bundle、projection target inventory、portal/shard future inventory を 1 つの suite として再現できます。`future/portal-worldlink/` と shard inventory は planned blueprint であり、final public grammar / ABI / WAN / distributed durable save-load / direct LLVM backend / final server-client split ではありません。
- `P-OPS-03` / `P-OPS-13` により operational room-chat host boundary は actualize 済みです:
  `samples/product-alpha1/operational/membership-chat/`、`crates/mir-ast::product_alpha1`、`crates/mir-runtime::product_alpha1_session`、`scripts/operational_product_samples.py` により、bounded room-oriented `ChatText("hello room") -> "room#lobby message accepted: hello room"` lane を `run-local` / `session` / `export-devtools` から observer-safe に再現できます。これは final room-chat service、multi-message chat transport、stdio builtin ではありません。
- `P-OPS-04` により operational Sugoroku behavior widening は actualize 済みです:
  `samples/product-alpha1/operational/sugoroku-world/`、`crates/mir-runtime::product_alpha1_session`、`scripts/operational_product_samples.py` により、bounded same-session roll / publish / witness / handoff / stale membership reject scenario を `run-local` / `session` / `export-devtools` / `release-check` から observer-safe に再現できます。これは final interactive game runtime、production networked gameplay、broader negative-row completion ではありません。
- `P-OPS-05` により operational projection manifest / packet / FFI schema は actualize 済みです:
  `samples/product-alpha1/operational/deployments/projection/projection.profile.json`、`crates/mir-ast::product_alpha1`、`crates/mir-runtime::product_alpha1_session`、`crates/mir-runtime::product_alpha1_devtools`、`scripts/operational_product_samples.py` により、schema-backed projection target / packet / FFI inventory を `check` / runtime plan / observer-safe devtools projection panel / helper `release-check` から再現できます。これは final server/client binary split、placement optimizer、direct LLVM backend completion ではありません。
- `P-OPS-06` により portal / world-link first cut は actualize 済みです:
  `samples/product-alpha1/operational/portal-worldlink/`、`crates/mir-ast::product_alpha1`、`crates/mir-runtime::product_alpha1_session`、`crates/mir-runtime::product_alpha1_devtools`、`scripts/operational_product_samples.py` により、bounded same-session resolve / handoff offer / witness emit / destination admit evidence を `check` / `run-local` / observer-safe devtools export / helper `release-check` から再現できます。`future/portal-worldlink/` blueprint は保持し、continuous spatial sync / WAN federation / final portal ABI は主張しません。
- `P-OPS-07` により two-shard hard-boundary first cut は actualize 済みです:
  `samples/product-alpha1/operational/two-shard-hard-boundary/`、`crates/mir-ast::product_alpha1`、`crates/mir-runtime::product_alpha1_session`、`crates/mir-runtime::product_alpha1_devtools`、`scripts/operational_product_samples.py` により、bounded same-session offer / prepare / commit / old-owner reject / missing-witness reject / stale-config reject evidence を `check` / `run-local` / observer-safe devtools export / helper `release-check` から再現できます。`future/two-shard-hard-boundary/` と `spatial-shard-future.profile.json` は保持し、gradient observation / continuous infinite federation / general model-check completion は主張しません。
- `P-OPS-09` により developer package authoring guide は actualize 済みです:
  `samples/product-alpha1/operational/templates/world-core-starter/`、`docs/hands_on/operational_package_authoring_01.md`、`docs/research_abstract/operational_package_authoring_01.md`、focused product-alpha tests、source hierarchy sync により、template-only starter を active operational roots と分離したまま `author -> check -> run-local -> session -> export-devtools -> view --check` の bounded authoring path を再現できます。generic scaffold CLI、final public grammar / ABI、arbitrary release helper generation は主張しません。
- `P-OPS-08` により backend feasibility inventory は actualize 済みです:
  `docs/hands_on/operational_backend_inventory_01.md`、`docs/research_abstract/operational_backend_inventory_01.md`、`plan/23`、`plan/50`、`plan/51`、`specs/26` により、current actualized backend-adjacent path が `native host launch bundle` だけであること、WASM client host と LLVM/native projection backend が docs-first inventory に留まること、packet/FFI/projection と auth/membership/capability/witness lane preservation が future reopen prerequisite であることを明示できます。generic backend build helper や direct codegen claim は追加しません。
- `P-OPS-10` により broader operational template catalog first cut は actualize 済みです:
  `samples/product-alpha1/operational/templates/membership-chat-starter/`、`samples/product-alpha1/operational/templates/sugoroku-world-starter/`、`docs/hands_on/operational_package_authoring_01.md`、`docs/research_abstract/operational_package_authoring_01.md`、focused product-alpha tests、source hierarchy sync により、validated starter catalog を `world_core` から `membership_chat` / `sugoroku_world` まで広げ、dependency-retarget obligation を明示した bounded authoring path を再現できます。portal/shard starter や generic scaffold CLI は主張しません。
- `P-OPS-11` により gradient observation profile inventory は actualize 済みです:
  `samples/product-alpha1/operational/future/gradient-observation.profile.json`、`samples/product-alpha1/operational/future/spatial-shard-future.profile.json`、`docs/hands_on/operational_gradient_observation_profile_01.md`、`docs/research_abstract/operational_gradient_observation_profile_01.md`、source hierarchy sync により、observer-only shard overlap reading、freshness fields、replication non-default reading、fallback behavior を `planned_only` inventory として固定できます。gradient observation runtime、portal/shard starter、general model-check completion は主張しません。
- `P-OPS-12` により portal/shard starter boundary は actualize 済みです:
  `docs/hands_on/operational_portal_shard_starter_boundary_01.md`、`docs/research_abstract/operational_portal_shard_starter_boundary_01.md`、`specs/26..27`、`plan/51..52`、authoring/dashboard sync により、validated starter catalog は intentional に `SugorokuWorld` で止め、portal/shard authoring は active executable roots を使い、`future/` inventory は non-executable のまま保つ current decision を reader-facing に固定できます。portal/shard starter templates、gradient observation runtime は主張しません。
- practical alpha-1 line は引き続き promoted implementation memory ですが、現在の closeout 群は **first-floor toolchain** です:
  `RUN-01..04`、`HP-A1-01..07`、`TR-A1-01..07`、`VIS-A1-01..07`、`SL-A1-01..03`、`AV-A1-01..03`、`PE2E-01..09` は first-floor evidence として読むべきであり、same-session operational α-0.5 / α-0.8 / α-0.9 ではありません。
- alpha-0 line は引き続き closed evidence reference です:
  Stage A..F は current-scope evidence として保持し、運用上の α-0.5 / α-0.8 / α-0.9 readiness と混同しません。

## ordered self-driven packages

| Order | Package | Macro / stage | Objective | Close condition | Rough estimate |
|---:|---|---|---|---|---|
| 1 | maintenance / dashboard freshness | `Macro 0`, `S6` | docs / dashboard / validator freshness を room-chat widening 後の current queue に合わせる | source hierarchy / docs scaffold / diff / report が current queue と一致する | small |
| 2 | gradient observation runtime widening | `Macro 8`, `S2/S3` | observer-only profile inventory から bounded runtime widening を reopen するかを切り分ける | profile-only のまま保つか、new bounded runtime evidence package を開くかの close condition を置く | medium |
| 3 | final-public gate scoping | `Macro 8+`, `S0/S1` | final grammar / ABI / WAN / distributed save-load / packaging adoption target のうち次に開く gate を選ぶ | user または explicit research prompt が next promoted line を選ぶ | user decision |

## current recommendation

- recommended reopen point:
  maintenance / dashboard freshness
- recommendation reason:
  `P-OPS-13` で current `MembershipChat` lane は bounded room-oriented `ChatText` まで widen されたため、次は docs / dashboard / validator wording を current queue に揃え、その次に gradient observation runtime widening を切り分けるのが自然である
- stop line:
  final public parser / viewer / telemetry ABI、distributed durable save/load、WAN/federation、arbitrary native execution、final product claim へ踏み込まない

## research-discovery items

| Item | Impact | Main options | Current recommendation |
|---|---|---|---|
| proof side discharge granularity | `plan/48` と external prover bridge に影響 | obligation family を coarse に束ねる / static-row ごとに細かく切る | まずは `specs/19` の residual obligation carrier を保ち、proof target 側で coarse-to-fine に展開する |
| distributed durable save/load line | `specs/20` の後段 family に影響 | product alpha local/Docker R0/R2 のまま保つ / distributed durable R3/R4 family を reopen する | current promoted reopen point では扱わない。product alpha release candidate 後の explicit final-public / durability gate としてだけ再評価する |
| auth policy catalog breadth | `specs/21` と host/runtime package line に影響 | minimal stdlib-like initial set / broader policy catalog | minimal initial set から始め、policy breadth は same-session attach line の実 evidence に合わせて widen する |
| integrated practical workflow boundary | `specs/18` の practical α-1 読みに影響 | one bounded workflow carrier / final public toolchain claim | `P-A1-23` で bounded workflow carrier は作成済み。final public toolchain claim はしない |
| product checker finite fragment breadth | `specs/25` と `P-A1-26/27/28` に影響 | existing practical rows only / product demo finite fragmentを少し拡張 | product demoに必要な package schema、effect/failure、capability/witness、message recovery、savepoint policy だけを bounded に足す |
| product transport command breadth | release validation に影響 | local-only first / local + Docker Compose TCP | `P-A1-29` で local + Docker Compose TCP first cut を実装し、`P-A1-31` release check で再確認済み |
| admin/debug product viewer breadth | final public viewer / telemetry ABI に影響 | observer-safe + kept-later marker / bounded admin debug panel | product alpha release candidate では concrete observer-safe static viewer と admin/debug `kept_later` marker に留める。full admin/debug view は final-public gate |
| operational room-chat beyond bounded `ChatText` | `MembershipChat` と `P-OPS-04+` の room-level behavior に影響 | current bounded `ChatText` lane を維持 / multi-message room-chat surface へ widen / transport-coupled chat lane を先に入れる | current recommendation は bounded room-oriented `ChatText` lane を維持し、transport-coupled chat や broader multi-message surface は later package に分ける |
| operational Sugoroku widening beyond bounded carrier | `P-OPS-07+` の domain realization に影響 | current deterministic scenario 維持 / broader interactive controls を追加 / negative rows を増やす | current recommendation は `P-OPS-04` の bounded scenario を維持し、shard line を先に進めてから broader controls を reopen する |
| projection inventory widening beyond current summary | future backend inventory と eventual split planning に影響 | current summary のまま保つ / richer projection IR を導入 / placement planner adjacent IR を別置きする | current recommendation は `P-OPS-05` の schema-backed summary を維持し、actual server/client split work が始まるまで richer IR は開かない |
| post-two-shard widening order | `specs/27` と future suite line に影響 | maintenance/dashboard freshness first / gradient observation runtime first / portal-shard starter revisit later | current recommendation は maintenance/dashboard freshness を先に閉じ、その次に gradient observation runtime widening を置き、portal/shard starter revisit はその後段または不要のまま保つ |

## user decision items

| Item | Impact | Main options | Current recommendation |
|---|---|---|---|
| `U1` beyond alpha packaging / host target / shipped surface | final public product line | repo-local alpha / installed binary / hosted service / other | product alpha-1 の defaults は `specs/25` で固定済み。final public surface はまだ固定しない |
| final shared-space operational catalog breadth | product/public scope | product alpha narrow showcase / broader final product line | product alpha-1 では narrow showcase を採る。broader final catalog は product alpha release candidate 後の user/final decision として残す |
| final public grammar / ABI | final public product line | alpha `package.mir.json` を進化させる / textual grammar を固定する | product alpha-1 では固定しない。alpha package format は migration可能と明記する |
| hosted service / production WAN | final public product line | local/Docker alpha / hosted service / WAN federation | product alpha-1 では local/Docker に限定する |

## self-driven maintenance tasks

| Task | Objective | Validation | Report requirement | Stop line |
|---|---|---|---|---|
| docs freshness audit | `README.md`、`Documentation.md`、`progress.md`、`tasks.md`、`samples_progress.md`、index docs を current queue に合わせる | `python3 scripts/check_source_hierarchy.py`、`python3 scripts/validate_docs.py`、`git diff --check` | new `docs/reports/NNNN-*.md` | snapshot docs で新しい規範判断を勝手に増やさない |
| runnable dashboard refresh | sample status、validation timestamp、operational gap を evidence-backed に保つ | relevant helper closeout commands | report + `samples_progress.md` | conceptual-only row を workflow-ready と書かない |
| operational suite guide refresh | `samples/product-alpha1/operational/`、hands-on、research summary、helper CLI surface を同じ package で同期する | `python3 -m unittest scripts.tests.test_operational_product_samples`、`python3 scripts/operational_product_samples.py check-all --format json` | report if touched | shard planned-only inventory や portal continuous sync を runnable claim に昇格しない |
| Rust formatting / regression repair | docs-only package でも formatting floor を崩さない | `cargo fmt --check`、affected tests | report if touched | unrelated feature workを混ぜない |

## non-promoted references

- `P-A0-23` / Stage B local runtime closeout は current-scope evidence reference であり、operational α-0.5 same-session runtime package ではない
- `P-A0-25` / Stage D lifecycle closeout は current-scope evidence reference であり、operational α-0.8 same-session hot-plug runtime package ではない
- `P-A0-26` / Stage E devtools closeout は current-scope evidence reference であり、`P-A1-22` bounded operational α-0.9 session-bound devtools package とは別 category である
