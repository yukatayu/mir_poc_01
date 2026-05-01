# progress

最終更新: 2026-05-01 11:53 JST

## この文書について

- この文書は repo 全体の **rough progress snapshot** です。
- 規範判断の正本は `specs/`、長期参照は `plan/`、実行証跡は `docs/reports/`、runnable sample dashboard は `samples_progress.md` です。
- 進捗率は **repo-local alpha-ready current layer** と **Mirrorea future-axis docs-first / sample-first integration** に scoped した rough estimate です。final public completion ではありません。
- 古い package 履歴の詳細は `docs/reports/` と `plan/90-source-traceability.md` を参照します。この snapshot では current checkpoint / next gate / validation floor を優先します。

## project axis

```text
正しい理論に基づき、
正しく hot-plug でき、
Place をまたいで実行・通信・検証・可視化できる
仮想空間システムを作る。
```

この軸は Mir / Mirrorea / PrismCascade / Typed-Effect Wiring Platform の separability を消すものではありません。

## current snapshot

- active floor:
  `samples/clean-near-end/`、`samples/current-l2/`、`samples/lean/`、Sugoroku world vertical slice、avatar follow representative slice は runnable です。
- Mir current-L2:
  finite-index first strong typing、order / handoff relation family、model-check second line、Lean foundation / generated stub、parser-free helper stack が repo-local に検証できます。
- Mirrorea carrier / runtime floor:
  `TermSignature`、`LayerSignature`、`MessageEnvelope` / `AuthEvidence`、typed visualization / telemetry envelope、`MembershipRegistry`、`PlaceCatalog`、`LogicalPlaceRuntimeShell`、principal-derived `ParticipantPlace[{principal}]` helper、engine-neutral `HotPlugRequest` / `HotPlugVerdict`、runtime-side `HotPlugRuntimeSkeletonReport` / `HotPlugRuntimeEngineReport` まで actualize 済みです。
- Preview / generated evidence floor:
  typed external synthetic preview、network helper-local canary、projection preview、projection/codegen committed generated bridge evidence、viewer typed public prototype inventory、storage / LLVM guardrail が current scope close 済みです。projection/codegen current `equivalence` reading は committed generated manifest と helper/report-local anchor の review-category alignment inventory に留まり、checker / proof / final public emitted-program ABI は kept-later gate に残ります。
- Hot-plug package floor:
  `P19` / `P20` / `P21` の narrow Rust-side floor は close 済みです。post-`P21` later-family docs-first trilogyも close 済みで、third recommendation の stop line は `freeze prerequisite fixed; public ABI still unfrozen` です。
- Current remaining open gate:
  追加の self-driven post-`P21` docs-first family は残っていません。次は actual `U1` commitment であり、installed binary / packaging target、host integration target、first shipped public surface scope、final shared-space operational catalog breadth の user-facing decision が必要です。
- Current maintenance lane:
  stale docs cleanup、validation rerun、report discipline、guardrail maintenance、formatting / regression repair は自走可能です。これは active maintenance であり、新しい implementation / product-shaping line ではありません。
  `scripts/current_l2_guided_samples.py` の active compatibility front door は `list / smoke-all / closeout` only です。legacy bundle / lane / reserve / hold-line / emit-* helper command claims、pre-clean-near-end prototype labels、old example/spec cluster detail は historical memory として `docs/reports/` / relevant `plan/` / `tasks.md` に委譲し、active command claim へ戻しません。
  2026-04-29 以降の formatting cleanup、guided-helper cooling、example/spec cluster cooling、front-door wording cooling、`1051` 以降の guardrail / snapshot / validation follow-up packages は maintenance-only closeout です。2026-05-01 11:01 JST の corrected full validation floor も pass 済みで、current blocker になる maintenance line はこの family には残っておらず、new implementation queue も reopened していません。

## strict non-claims

- standard I/O は Mir core primitive ではありません。
- `auth none` baseline は final auth design ではありません。
- helper-local preview / report-local inventory / committed generated bridge evidence は final public API ではありません。projection/codegen current `equivalence` reading も review-category alignment inventory に留まり、cross-place equivalence checker / proof completion / final public emitted-program ABI ではありません。
- current `VerificationLayer` emitted rows / previews / downstream consumers / emitted verifier handoff artifacts は separate current surfaces であり、それだけで final public verifier contract を意味しません。
- runtime-private hot-plug request / verdict / engine-state names は final public hot-plug ABI names ではありません。
- visualization / telemetry は untyped debug leak ではなく、label / authority / redaction / retention を持つ typed information-bearing effect として扱います。
- `U1` option matrix は actual product decision ではなく、actual decision の入口です。

## 3-axis progress

| layer / track | 論理仕様 | ユーザ向け仕様 | 実装 / 運用 | 自走可否 | 現在の読み |
|---|---:|---:|---:|---|---|
| Mir core / current-L2 | 90% | 88% | 80% | 着手可能 | finite-index current layer は強い。final parser / public API は未完 |
| order / handoff / cut family | 90% | 90% | 80% | 着手可能 | high-level relation line は runnable。public artifact contract は未完 |
| theorem / model-check / Lean | 92% | 90% | 86% | 着手可能 | repo-local bridge は強い。production prover / model-check binding は未完 |
| shared-space samples | 84% | 87% | 75% | 着手可能 | Sugoroku / avatar の runnable floor はある。real transport / durable evidence は未完 |
| Mirrorea runtime / fabric carrier | 86% | 92% | 82% | public contract 以外は着手可能 | core carrier、membership/place substrate、hot-plug request/verdict、runtime engine-state narrow floor まで actualize |
| typed external / projection / viewer | 82% | 91% | 82% | public contract は要仕様確認 | helper preview、generated bridge evidence、typed viewer inventory はある。final host / viewer / emitted executable family は未完 |
| hot-plug later-family boundary | 82% | 91% | 78% | actual ABI は要仕様確認 | docs-first trilogy close 済み。rollback / durable migration / distributed ordering / final public ABI は completion claim なし |
| storage / backend guardrail | 82% | 91% | 88% | 着手可能 | external workdir / cleanup / LLVM staging visibility は closeout 済み。actual LLVM build と backend choice は未決 |
| docs / dashboard / repository structure | 94% | 94% | 88% | 着手可能 | source hierarchy、snapshot docs、dashboard semantics は current line に追随。履歴の圧縮は継続保守対象 |

## macro phase map

| Macro phase | 主眼 | 現在位置 | 重さ | 自走可否 |
|---|---|---|---|---|
| `Macro 0` | repository memory / docs / traceability | active maintenance | low-medium | 着手可能 |
| `Macro 1` | semantic kernel / invariant stabilization | late current-L2 | medium | 着手可能 |
| `Macro 2` | parser-free validation substrate | late | medium | 着手可能 |
| `Macro 3` | compile-ready minimal actualization | late | medium | public parser / API 以外は着手可能 |
| `Macro 4` | executable sample floor | active clean suite | medium | 着手可能 |
| `Macro 5` | theorem / model-check bridge | repo-local alpha-ready | medium-high | public seam 以外は着手可能 |
| `Macro 6` | shared-space / fabric boundary | P21 narrow floor closed | high | real network / public contract 以外は着手可能 |
| `Macro 7` | toolchain / backend / public operational interface | guardrail + prototype inventory closed | high | repo-side guardrail は着手可能。packaging / shipped surface は要仕様確認 |
| `Macro 8` | application realization / product commitment | U1 actual choice gate | high | 要仕様確認 |

## feature family snapshot

| feature family | 現在地 | できていること | まだ残ること |
|---|---|---|---|
| executable sample corpus | current scope close | active clean suite、Sugoroku、avatar、dashboard | final public sample catalog |
| finite-index typing / order-handoff | current scope close | user-defined finite theory、publication / witness / handoff relation | final source principal wording、final emitted-artifact schema、public checker contract、public witness/provider/emitted-handoff contract |
| theorem / model-check / Lean | current scope close | model-check second line、small Lean proof、generated stub | full domain discharge と production binding |
| shared-space runtime samples | current scope close | attach / membership / handoff / late join / follow / fallback / reset safety | real transport、durable distributed commit、public runtime API |
| typed external / network / projection preview | first cuts closed | host-boundary preview、NET canaries、projection preview、generated bridge manifest | final host schema、real transport、final emitted executable family |
| verification / visualization composition | first cuts closed | typed view / telemetry envelope、viewer prototype inventory、fail-closed route trace、helper `verification_handoff_witness` / runtime `verification_model_check` emitted floor | theorem bridge / runtime policy widening contract、final viewer / verifier API、retention policy、telemetry service |
| hot-plug runtime package | P21 + docs-first trilogy closed | helper lifecycle, request/verdict carrier, runtime engine-state narrow floor, three later-family boundaries | rollback protocol, durable migration engine, distributed activation ordering, final public ABI |
| storage / backend guardrail | first cut closed | external workdir, cargo target/cache binding, LLVM staging visibility, cleanup guard | actual LLVM build, backend target, packaging |

## closed package memory and active gate

| Item | Status | Objective | Stop line |
|---|---|---|---|
| `P0..P18` repo-side integration packages | closed | current-L2 / Mirrorea future-axis docs-first and sample-first floor | final public product completion ではない |
| `P19` hot-plug request/verdict carrier | closed | engine-neutral carrier in `mirrorea-core` | helper IDs / runtime engine / public ABI へ広げない |
| `P20` runtime hot-plug skeleton | closed | consumer-side thin runtime/report assembly in `mir-runtime` | completed engine と呼ばない |
| `P21` runtime engine-state narrow cut | closed | admitted request/verdict + runtime snapshotから runtime-side engine state progression を narrow に actualize | rollback / migration / distributed ordering / public ABI を claim しない |
| post-`P21` rollback / durable migration family | closed docs-first | first recommendation boundary | actual rollback / durable migration engine completion ではない |
| post-`P21` distributed activation ordering family | closed docs-first | second recommendation boundary | actual distributed activation protocol ではない |
| post-`P21` final public hot-plug ABI family | closed docs-first | third recommendation bridge: `freeze prerequisite fixed; public ABI still unfrozen` | actual public ABI freeze ではない |
| `U1` actual commitment | open | packaging / host target / shipped surface / final catalog breadth を actual choice へ進める | user-facing decision なしに public freeze しない |
| docs / validation maintenance | active | stale wording removal、report sync、validation rerun、formatting cleanup | success claim は fresh validation 後に限る |

## validation anchors

- source hierarchy:
  `python3 scripts/check_source_hierarchy.py`
- docs scaffold:
  `python3 scripts/validate_docs.py`
- current-L2 / clean suite:
  `python3 scripts/current_l2_guided_samples.py closeout --format json`
  `python3 scripts/clean_near_end_samples.py closeout`
- Lean / theorem sync:
  `python3 scripts/current_l2_lean_sample_sync.py`
- representative runtime slices:
  `python3 scripts/sugoroku_world_samples.py closeout --format json`
  `python3 scripts/avatar_follow_samples.py closeout --format json`
- preview / generated / viewer / transport:
  `python3 scripts/typed_external_boundary_samples.py closeout --format json`
  `python3 scripts/network_transport_samples.py check-all --format json`
  `python3 scripts/projection_codegen_samples.py closeout --format json`
  `python3 scripts/visual_debugger_viewer_samples.py closeout --format json`
- Rust crates:
  `cargo test -p mir-ast`
  `cargo test -p mirrorea-core`
  `cargo test -p mir-runtime`
  `cargo test -p mir-semantics`
- formatting / diff:
  `cargo fmt --check`
  `git diff --check`

## recent log

- 2026-05-01 11:53 JST — network transport sample README / hands-on / `plan/22` の command-anchor drift を修正した。`check-all --format json` は `NET-02..05` executable canary anchor、`closeout --format json` は inventory evidence、`NET-01` は Sugoroku loopback parity anchor として分離した。
- 2026-05-01 11:42 JST — guided helper retirement audit で `current_l2_guided_samples.py` の live front door が `list / smoke-all / closeout` only であることを再確認し、`plan/00` / `plan/10` の pre-clean-near-end prototype / representative bundle wording を historical comparison memory へ冷却した。`samples/current-l2/` は base corpus、`samples/clean-near-end/` は active canonical executable suite として分けて記録した。
- 2026-05-01 11:31 JST — validate/docs scope wording audit 中に remaining active hands-on / research abstract の network transport executable anchor drift を修正した。`current_phase_closeout_01.md`、`public_api_parser_gate_01.md`、`mirrorea_future_axis_01.md` は `network_transport_samples.py check-all --format json` を executable validation anchor として示す。
- 2026-05-01 11:27 JST — latest-report guardrail commit 後の docs-focused freshness checkpoint を close した。clean tree で `python3 -m unittest scripts.tests.test_validate_docs`、`check_source_hierarchy.py`、`validate_docs.py`、`git diff --check` が pass し、latest report heading guardrail は current scaffold validation に入っている。
- 2026-05-01 11:21 JST — `validate_docs.py` の scaffold guardrail を latest numbered report 1 本へ narrow に拡張した。historical reports 全体の semantic lint には広げず、latest report が template required headings を欠く場合だけ fail し、historical report 欠落だけでは fail しない behavior-level unit tests を RED→GREEN で追加した。
- 2026-05-01 11:13 JST — report template compliance guardrail を narrow に補強し、`docs/reports/TEMPLATE.md` と `scripts/validate_docs.py` の required heading に `## Commands run` を追加した。TDD で `scripts/tests/test_validate_docs.py` の RED を確認後に GREEN 化し、template drift 防止を maintenance-only に閉じた。actual numbered report 全体の semantic lint ではない。
- 2026-05-01 11:04 JST — corrected network transport anchor 後の full validation freshness checkpoint を close した。16-command full floor は `NET-02..05` を `check-all --format json` で実行する corrected command set で全件 pass、Cargo crate tests / `cargo fmt --check` / `git diff --check` も pass。補助で Lean sync と storage guardrail も pass し、known `/mnt/mirrorea-work/llvm` root-owned warning 以外の新規 blocker はない。
- 2026-05-01 10:56 JST — network transport validation anchor を correction し、`check-all --format json` が `NET-02..05` canaries を実行する command、`closeout --format json` が inventory-only command であることを `progress.md` / `tasks.md` / `samples_progress.md` / report に反映した。`check-all` は 4/4 pass、production socket / durable replay claim はしていない。
- 2026-05-01 10:50 JST — recent log を再圧縮し、10:07..10:48 の active-doc / task-map / dashboard / storage maintenance package 詳細を `docs/reports/1062..1073` に委譲した。current floor、strict non-claims、`U1` open gate、validation anchors は維持し、new implementation queue は reopened していない。
- 2026-05-01 10:26..10:48 JST — full validation checkpoint、storage guardrail audit、task/progress/samples dashboard wording cooling、active-doc point-in-time wording repair、no-finding audit、maintenance-family compression を close した。known `/mnt/mirrorea-work/llvm` root-owned warning 以外の新規 blocker はなく、sample status / public surface / implementation queue は変えていない。
- 2026-05-01 10:07..10:18 JST — active front-door docs の snapshot drift、fixed-line command audit、sample validation-count wording、maintenance evidence band follow-up を close した。live queue authority は `progress.md` / `tasks.md` に残し、helper preview / report-local evidence を final public API として扱っていない。
- 2026-05-01 09:57 JST — `progress.md` current snapshot の maintenance-lane prose を再圧縮し、long example/spec cluster mapping を `docs/reports/` / relevant `plan/` / `tasks.md` に委譲した。active maintenance、`current_l2_guided_samples.py` front door、no blocker、no implementation queue reopened の事実は保持した。
- 2026-05-01 09:52 JST — `progress.md` recent log を snapshot 文書として再圧縮し、2026-04-28 以降の package-level chronology を `docs/reports/` と relevant `plan/` files へ委譲した。current floor、strict non-claims、`U1` open gate、validation anchors は維持し、new implementation queue は reopened していない。
- 2026-05-01 09:47 JST — `tasks.md` current task-level status を snapshot 文書として再圧縮した。active floor、closed package bands、maintenance lane、1051..1058 guardrail closeout、`U1` actual commitment を保持した。
- 2026-05-01 09:12..09:44 JST — guardrail maintenance band `1051..1058` を close: dashboard freshness audit、validator scope guardrail、full validation freshness checkpoint、target-specific warning cleanup、Makefile docs-check parity、report template discipline、template-heading guard、source hierarchy policy/spec coverage。これは maintenance-only closeout であり、sample semantics / public surface / implementation queue は変えていない。
- 2026-05-01 00:03..09:12 JST — front-door / hands-on / research-abstract / plan wording cooling band を close し、live queue / recommendation / completed-engine wording を repository-memory reading、snapshot pointers、strict non-claims へ分離した。hot-plug rollback / durable migration / distributed ordering / final public ABI は still-later。
- 2026-04-30 13:27..23:59 JST — current-line handoff normalization、plan / landing docs temperature audit、guided helper current-front-door cooling、active examples evidence refresh、VerificationLayer / projection / `FAIRY-05` / order-handoff / witness-provider docs-first inventories を maintenance / docs-first package 群として close した。`scripts/current_l2_guided_samples.py` の active front door は `list / smoke-all / closeout` only。
- 2026-04-29 — docs freshness dashboard full validation、uncommitted Rust formatting cleanup commit `b213721`、`P19` / `P20` / `P21` narrow hot-plug runtime packages、post-`P21` rollback / durable migration・distributed activation ordering・final public hot-plug ABI docs-first trilogyを close した。`freeze prerequisite fixed; public ABI still unfrozen` までであり actual `U1` は open。
- 2026-04-28 — future-plan integration / next package queue stabilization handoff mirror を close し、repo current line への同期を行った。以後の詳細証跡は `docs/reports/`、長期比較は `plan/`、sample dashboard は `samples_progress.md` を参照する。
