# progress

最終更新: 2026-04-29 12:02 JST

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
  typed external synthetic preview、network helper-local canary、projection preview、projection/codegen committed generated bridge evidence、viewer typed public prototype inventory、storage / LLVM guardrail が current scope close 済みです。
- Hot-plug package floor:
  `P19` / `P20` / `P21` の narrow Rust-side floor は close 済みです。post-`P21` later-family docs-first trilogyも close 済みで、third recommendation の stop line は `freeze prerequisite fixed; public ABI still unfrozen` です。
- Current next open work:
  追加の self-driven post-`P21` docs-first family は残っていません。次は actual `U1` commitment であり、installed binary / packaging target、host integration target、first shipped public surface scope、final shared-space operational catalog breadth の user-facing decision が必要です。
- Current maintenance lane:
  stale docs cleanup、validation rerun、report discipline、guardrail maintenance、formatting / regression repair は自走可能です。2026-04-29 には uncommitted Rust formatting cleanup を `b213721` として commit / push 済みです。

## strict non-claims

- standard I/O は Mir core primitive ではありません。
- `auth none` baseline は final auth design ではありません。
- helper-local preview / report-local inventory / committed generated bridge evidence は final public API ではありません。
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
| finite-index typing / order-handoff | current scope close | user-defined finite theory、publication / witness / handoff relation | final source principal wording と public checker contract |
| theorem / model-check / Lean | current scope close | model-check second line、small Lean proof、generated stub | full domain discharge と production binding |
| shared-space runtime samples | current scope close | attach / membership / handoff / late join / follow / fallback / reset safety | real transport、durable distributed commit、public runtime API |
| typed external / network / projection preview | first cuts closed | host-boundary preview、NET canaries、projection preview、generated bridge manifest | final host schema、real transport、final emitted executable family |
| verification / visualization composition | first cuts closed | typed view / telemetry envelope、viewer prototype inventory、fail-closed route trace | final viewer / verifier API、retention policy、telemetry service |
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
- representative runtime slices:
  `python3 scripts/sugoroku_world_samples.py closeout --format json`
  `python3 scripts/avatar_follow_samples.py closeout --format json`
- preview / generated / viewer / transport:
  `python3 scripts/typed_external_boundary_samples.py closeout --format json`
  `python3 scripts/network_transport_samples.py closeout --format json`
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

- 2026-04-29 12:02 JST — docs freshness dashboard audit の full validation を rerun し、source hierarchy / docs scaffold / current-L2 / clean near-end / Sugoroku / avatar / typed external / network / projection / viewer / storage scripts / `mir-ast` / `mirrorea-core` / `mir-runtime` / `mir-semantics` / `cargo fmt --check` / `git diff --check` が pass した。`validate_docs.py` は report `0998` 作成後に 996 numbered reports を確認した。
- 2026-04-29 11:50 JST — docs freshness audit を開始し、`progress.md` / `tasks.md` / `samples_progress.md` / `docs/research_abstract` を current snapshot として再圧縮する方針にした。uncommitted Rust formatting cleanup は先に `cargo fmt --check`、`cargo test -p mir-ast`、`cargo test -p mirrorea-core`、`cargo test -p mir-runtime --test hotplug_runtime_skeleton`、`cargo test -p mir-runtime --test clean_near_end_samples`、`git diff --check` で確認し、commit `b213721` として push 済み。
- 2026-04-29 04:47 JST — post-`P21` `final public hot-plug ABI` family third recommendation を docs-first package として close し、`freeze prerequisite fixed; public ABI still unfrozen / next open work = actual U1 commitment` に同期した。Sugoroku closeout、`cargo test -p mir-runtime --test hotplug_runtime_skeleton`、`check_source_hierarchy.py`、`validate_docs.py`、`git diff --check` を fresh rerun した。
- 2026-04-29 04:16 JST — post-`P21` `distributed activation ordering` family second recommendation を docs-first package として close した。helper `activation_cut` は distributed activation ordering proof ではないという stop line を同期した。
- 2026-04-29 03:23 JST — post-`P21` `rollback / durable migration` family first recommendation を docs-first package として close した。rejected `detached_roll_request#1` と `migration_contract.status = deferred` を completed rollback / migration と混同しない boundary を同期した。
- 2026-04-29 03:07 JST — `P21` runtime-crate hot-plug completed-engine narrow cut を close し、`HotPlugRuntimeEngineState` / `HotPlugRuntimeEngineReport` と consumer-side `assemble_hotplug_runtime_engine_report()` を actualize した。runtime-side state progression は narrow / non-public のまま。
- 2026-04-29 00:41 JST — `P19` hot-plug request/verdict carrier tranche を close し、`mirrorea-core` に engine-neutral carrierを actualize した。
- 2026-04-29 01:13 JST — `P20` runtime hot-plug orchestration skeleton first tranche を close し、`mir-runtime` に thin runtime/report assembly を actualize した。
- 2026-04-28 03:27 JST — `sub-agent-pro/mirrorea_next_stage_full_plan_handoff_2026-04-27.md` を repo current line に mirror し、future-plan integration / next package queue stabilization を close した。
