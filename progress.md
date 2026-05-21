# progress

最終更新: 2026-05-21 19:22 JST

## この文書について

- この文書は repo 全体の **operational workflow snapshot** です。
- 規範判断の正本は `specs/`、長期参照は `plan/`、実行証跡は `docs/reports/`、runnable sample dashboard は `samples_progress.md` です。
- 進捗率は primary metric ではありません。`100%` は外部開発者がその layer を実際に使える operational workflow または product/public layer だけに使います。
- helper / sidecar / report / expected JSON / first-floor runner は completion ではなく evidence として分類します。
- package ごとの履歴は `docs/reports/` と `plan/90-source-traceability.md` を参照し、この snapshot では current checkpoint / next gate / validation floor を優先します。

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
  `P-COMP-01` Mir computational sample scaffold actualization
- current promoted reopen point:
  `P-POSE-01` Transform / PoseGraph scaffold
- current reading:
  repo は bounded operational α-0.5 / α-0.8 / α-0.9、bounded practical α-1 integrated workflow、product alpha release-candidate workflow、installed-binary adoption probe、canonical operational product sample suite まで reproducible な workflow evidence を持つ。その上で、current typed external `AddOne` は host-boundary evidence であり、Mir-owned arithmetic / variables / arrays / records / control-flow completion ではない。
- practical usability:
  external developer が clean clone から documented commands で `mirrorea-alpha` product demo と operational suite を check / run / attach / save / quiescent-save / transport / export-devtools / view / build-native-bundle まで再現する段階には達している。これは **controlled alpha use** として実用可能という意味であり、final public product / final SDK / hosted service ではない。
- self-driven status:
  operational runtime widening queue は exhausted のまま。broader distribution / final shared-space operational catalog breadth は user-spec-required gate として残す。一方、docs/spec の current self-driven line は front-half `P-COMP-01 -> P-POSE-01 -> P-PROJ-01 -> P-ENG-01 -> front-half closeout`、implementation half `P-COMP-02 -> P-COMP-03 -> P-COMP-04 -> P-POSE-02` へ整理した。`P-COMP-01` は planned-only computational root / matrix / helper / tests まで actualize 済みで、current next package は `P-POSE-01` である。`specs/32` / `plan/57` により、一度実行を依頼された後は package-by-package で止まらず進む autonomous execution contract も固定した。

## workflow-readiness axes

| 軸 | Workflow reading | Current status |
|---|---|---|
| 論理仕様 | boundary-fixed, not workflow completion by itself | `specs/18..31` が practical / operational / product alpha / operational sample / portal-shard / computational core / PoseGraph / projection-backend / engine-adapter boundary を分ける。final public grammar / ABI は未固定 |
| ユーザ向け仕様 | reproducible workflow guidance exists | `README.md`、`Documentation.md`、`docs/hands_on/product_alpha1_01.md`、`docs/hands_on/operational_product_sample_01.md`、authoring / backend / portal-shard guides が current command path と non-claims を説明する |
| 実装 / 運用 | product alpha release-candidate plus canonical operational suite | product demo and operational suite are runnable through local/Docker controlled validation; installed-binary + generated host launch bundle is the current adoption probe. Computational / PoseGraph samples are planned-only |

## line snapshot

| Line | Category | Workflow status | Current status | Next gap |
|---|---|---|---|---|
| current-L2 active floor | runnable evidence | evidence-backed runnable floor | `samples/clean-near-end/`、`samples/current-l2/`、`samples/lean/` は active roots として維持 | final public parser/API |
| Spaces alpha-0 | evidence line | evidence-closed only | `samples/alpha/` と Stage A..F は current-scope evidence | operational workflow completion とは別 |
| practical alpha-1 first floors | first-floor evidence | evidence-closed only | front-door / checker / runtime / hot-plug / transport / devtools / save-load / preview families are validated evidence | product/public-ready α-1 とは別 |
| practical alpha-1 integrated workflow | bounded workflow line | developer-reproducible bounded workflow | `scripts/practical_alpha1_integrated_workflow.py check-all --format json` で再現 | final public product claim |
| product alpha-1 release candidate | product alpha workflow | release-candidate ready, not final product | `scripts/product_alpha1_release_check.py --format json check-all --out <dir>` が Docker 込みで accepted なら release-candidate evidence | broader public distribution / final catalog decision |
| installed-binary adoption probe | public-ish alpha adoption probe | bounded adoption candidate | `scripts/product_alpha1_installed_binary_check.py --format json check-all --out <dir>` が built binary + generated host bundle replay を確認 | archive / installer / hosted-service shape |
| operational product sample suite | canonical operational suite | workflow-ready canonical suite, not final product | `scripts/operational_product_samples.py check-all --format json` が six roots, attach packages, transport, devtools, native bundle, portal/shard/gradient cuts を確認 | user-spec-required broader distribution / final catalog decision |
| Mir computational core | docs/spec rebaseline | planned-only scaffold actualized | `specs/28` / `plan/53` define first floor and package gates; `samples/product-alpha1/computational/` and `scripts/mir_computational_samples.py` now classify planned rows and reject `run` as `planned_only` | `P-COMP-02` pure AddOne in Mir after front-half closeout |
| Transform / PoseGraph | docs/spec rebaseline | boundary-fixed, planned samples only | `specs/29` / `plan/54` define pose snapshot, anchor, no-split-frame, save/load/devtools hooks | PoseGraph scaffold and positive/negative no-split-frame evidence |
| projection/backend boundary | docs/spec rebaseline | boundary-fixed, inventory-only | `specs/30` / `plan/55` keep projection as target / packet / FFI inventory, not codegen | server/client/adapter manifest inventory and report surface |
| engine/WASM/FFI adapter boundary | docs/spec rebaseline | boundary-fixed, inventory-only | `specs/31` / `plan/56` keep engines/providers under typed adapter contracts | provider contract inventory, no arbitrary execution |
| autonomous execution contract | docs/spec execution policy | boundary-fixed | `specs/32` / `plan/57` define no-question execution, front-half closeout, package close protocol, sub-agent review, and validation cadence | continue `P-POSE-01` / `P-PROJ-01` / `P-ENG-01` front-half queue |

## subsystem status

- **Mir**
  current-L2 の semantics / invariant / parser-free evidence floor は維持。Mir-owned computational core first floor は docs/spec rebaseline 済みだが、runtime/sample implementation は未着手。final textual grammar と final public checker/runtime/verifier API は未固定。
- **Mirrorea**
  same-session carrier、hot-plug、local/Docker transport、observer-safe devtools、local R0/R2 save-load、native host launch bundle の alpha workflow は再現可能。WAN/federation、distributed durable save-load、final viewer/telemetry ABI は後段。
- **Typed-Effect Wiring Platform**
  `AddOne` と bounded `ChatText` は typed external host boundary evidence。`AddOne` は Mir-owned computation completion ではない。stdio builtin は導入しない。
- **PrismCascade / Reversed Library**
  separable kept-later line。current alpha-1 operational workflow の実装対象ではない。

## macro phase map

| Macro | 主眼 | 現在位置 | 重さ | 自走可否 |
|---|---|---|---|---|
| `Macro 0` | repository memory / docs / traceability | snapshot docs を current gate に圧縮中 | light | 着手可能 |
| `Macro 1` | semantic kernel / invariant / boundary stabilization | computational core / effect split / PoseGraph boundary rebaseline in progress | medium | 着手可能 |
| `Macro 5` | theorem / model-check / external verifier bridge | obligation export boundary fixed; broad discharge later | medium | 着手可能 |
| `Macro 6` | distributed fabric / shared-space / runtime evolution boundary | same-session + local/Docker alpha workflow; production distributed line later | heavy | 後段依存 |
| `Macro 7` | toolchain / backend / developer surface / public operational interface | product alpha release-candidate and installed-binary adoption probe are reproducible; projection/backend remains inventory | heavy | 着手可能 for docs/spec, 要仕様確認 for shipped surface |
| `Macro 8` | domain / application realization | operational suite six-root chain is runnable; PoseGraph and final catalog breadth open | heavy | 着手可能 for PoseGraph docs/spec, 要仕様確認 for final catalog |

## feature maturity rows

| Feature | Workflow status | 読み | 着手可否 |
|---|---|---|---|
| multi-node / fabric | bounded local/Docker alpha workflow | same-session plus controlled local/Docker TCP evidence exists; production WAN is not claimed | 後段依存 |
| robustness via contracts / theorem / model-check boundary | boundary-fixed | static checker / model-check / proof-side stratification is fixed; broad proof discharge remains later | 着手可能 |
| dynamic attach / detach / DAG-safe evolution | bounded same-session attach workflow | debug/auth/rate-limit/object/avatar attach behavior is visible; accepted detach and distributed ordering remain later | 着手可能 |
| `atomic_cut` and higher-level ordering | semantics fixed, evidence-backed | `atomic_cut` remains place-local rollback frontier; durable/distributed commit is not implied | 着手可能 |
| executable sample corpus | workflow-ready where scoped | clean near-end, practical alpha workflows, product alpha release candidate, installed-binary probe, and operational suite have runnable anchors | 着手可能 |
| Mir-owned computation | boundary-fixed, planned-only samples | current alpha host `AddOne` is external adapter evidence only; pure AddOne in Mir is next proof point | 着手可能 |
| PoseGraph / no-split-frame | boundary-fixed, planned-only samples | same-client same-observation-snapshot invariant, `Anchor`, `AnchorBinding`, `AnchorSwitch`, and stale-anchor reacquire gates are defined; no runtime sample yet | 着手可能 |

## current blockers

- broader installed/public distribution is undefined beyond developer-built `mirrorea-alpha` plus locally generated native host launch bundle.
- final shared-space operational catalog breadth is undefined beyond the bounded product alpha-1 narrow showcase.
- Mir-owned computational core runtime evidence is not yet implemented. `P-COMP-01` added only the planned-only scaffold root / helper / matrix.
- PoseGraph save/load / devtools carrier is not yet implemented.
- projection/backend boundary is inventory-only; no server/client split or codegen exists.
- engine/WASM/FFI adapter boundary is inventory-only; no arbitrary execution is admitted.
- backend realization, bounded native/WASM provider admission, and final engine adapter ABI remain user-spec-required / kept-later gates.
- final public grammar / ABI / SDK, final viewer / telemetry ABI, hosted service, WAN/federation, and distributed durable save-load remain later gates.
- current user-required decision:
  `U1_beyond_alpha_packaging_host_target_shipped_surface` and `final_shared_space_operational_catalog_breadth` remain for final distribution, but they no longer block the docs/spec computational-core line.

## validation floor

Use the focused all-up anchors first:

```bash
python3 -m unittest scripts.tests.test_mir_computational_samples
python3 scripts/mir_computational_samples.py check-all --format json
python3 -m unittest scripts.tests.test_validate_docs scripts.tests.test_product_alpha1_installed_binary_check scripts.tests.test_product_alpha1_release_check scripts.tests.test_operational_product_samples
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release
python3 scripts/product_alpha1_installed_binary_check.py --format json check-all --out /tmp/mirrorea-alpha1-installed-binary-check
python3 scripts/operational_product_samples.py check-all --format json
```

Use narrower command families from `samples_progress.md` or the hands-on guides when debugging a specific layer.

## recent log

- 2026-05-21 19:22 JST
  `P-COMP-01` で `samples/product-alpha1/computational/`、`matrix.json`、`scripts/mir_computational_samples.py`、unit test、validator registration、snapshot docs を actualize し、`run comp-02-pure-add-one` が `planned_only` で拒否される current non-claim を machine-readable に固定した。next reopen point は `P-POSE-01`。
- 2026-05-21 18:56 JST
  `P-COMP-00B` で autonomous execution contract を reviewer findings と同期。front-half `P-COMP-01 -> P-POSE-01 -> P-PROJ-01 -> P-ENG-01 -> closeout` と implementation half `P-COMP-02 -> P-COMP-03 -> P-COMP-04 -> P-POSE-02` を分け、`mir-semantics` computational module、manifest/provider compatibility、provider rollback/replay/cut policy、user-spec-required backend/native/WASM gates を明記した。
- 2026-05-21 17:35 JST
  `P-COMP-00` で computational-core drift を rebaseline。Product Alpha-1 workflow は保持しつつ、current `AddOne` を typed external host-boundary evidence に限定し、`specs/28..31` / `plan/53..56` で Mir-owned computation、PoseGraph、projection/backend、engine-adapter boundary を docs/spec line として追加した。runtime implementation は未着手。
- 2026-05-07 13:08 JST
  `P-OPS-27` で alpha-1 usability / docs snapshot audit を実施。product release check、installed-binary probe、operational suite check-all を Docker 込みで再確認し、overview docs を current status / next gate / validation anchors 中心へ圧縮した。全面 Python test で見つかった `mir_hilight.html` の active sample inventory drift も同期した。
- 2026-05-07 12:25 JST
  `P-OPS-26` で `user_final_decision_scope` を追加し、current delivery unit を developer-built binary + generated host launch bundle、current catalog scope を bounded product alpha-1 narrow showcase に固定した。broader final distribution / final shared-space catalog breadth は user-spec-required gate のまま。
- 2026-05-07 10:22-12:03 JST
  `P-OPS-20..25` で distribution scope、room-chat scope、portal/shard starter scope、Sugoroku scope、widening queue scope を machine-readable に固定し、current self-driven operational reopenings を non-promoted に戻した。
- 2026-05-06 21:12-2026-05-07 09:57 JST
  `P-OPS-01..19` で operational suite scaffold、room-chat, Sugoroku, projection, portal, hard shard, gradient observation, authoring starters, backend inventory, installed-binary probe, shipped surface を段階的に actualize / narrow した。
- 2026-05-05
  `P-A1-25..31` で product alpha boundary、CLI/schema、same-session runtime、save/load/quiescent-save、transport/devtools、native host bundle、release-check workflow を actualize した。
- Older history:
  detailed package chronology is intentionally kept in `docs/reports/` and repository memory under `plan/`.
