# progress

最終更新: 2026-04-27 23:26 JST

## この文書について

- この文書は repo 全体の **rough progress snapshot** です。
- 規範判断の正本は `specs/`、長期参照は `plan/`、runnable sample dashboard は `samples_progress.md`、詳細証跡は `docs/reports/` にあります。
- 進捗率は **repo-local alpha-ready current layer** と **Mirrorea future-axis docs-first / sample-first integration** に scoped した rough estimate であり、final public completion を意味しません。

## project axis

```text
正しい理論に基づき、
正しく hot-plug でき、
Place をまたいで実行・通信・検証・可視化できる
仮想空間システムを作る。
```

## current snapshot

- active floor:
  clean near-end suite、Sugoroku world vertical slice、avatar follow representative slice は runnable
- future-axis floor:
  `TermSignature`、`LayerSignature`、`MessageEnvelope / Auth seam`、`VisualizationProtocol`、typed external synthetic preview helper、projection preview、hot-plug helper-local lifecycle canary、network transport helper-local canary までは actualize 済み
- docs / planning floor:
  `samples_progress.md`、`docs/hands_on/current_phase_closeout_01.md`、`plan/19-repository-map-and-taxonomy.md` が current snapshot の入口として揃っている
- current promoted next line:
  Typed external boundary residual planned family review
- next reopen point:
  Projection / placement residual emitted-program gate
- still later:
  real transport、final public auth / visualization / projection / hot-plug API、packaging / FFI / engine adapter、actual LLVM build

## 現在の一言での読み

2026-04-27 時点の repo は、**current-L2 と current shared-space sample の runnable floor を維持したまま、Mirrorea future-axis を helper-local / report-local preview と docs-first gate で段階 actualize し、次に phase 9 residual planned family review と phase 12 residual emitted-program gate を整理する段階**です。

## 3 軸 progress

| layer / track | 論理仕様 | ユーザ向け仕様 | 実装 / 運用 | 現在の読み |
|---|---:|---:|---:|---|
| Mir core / current-L2 | 90% | 88% | 80% | finite-index current layer は強いが、final parser / public API は未完 |
| order / handoff / cut family | 90% | 90% | 80% | high-level relation line は runnable。final source wording と public artifact contract は残る |
| theorem / model-check boundary | 92% | 90% | 86% | repo-local bridge は強いが、concrete external binding は未完 |
| Lean foundations / proof spine | 86% | 88% | 68% | small proof fragment と generated stub はあるが、full discharge ではない |
| shared-space samples | 84% | 86% | 74% | Sugoroku / avatar の runnable floor はあるが、real transport / durable evidence は未着手 |
| docs / dashboard / repository structure | 90% | 92% | 82% | front-door docs、snapshot docs、layer map、sample dashboard は current line に追随 |
| Mirrorea future axis | 80% | 90% | 68% | helper-local / report-local preview は揃ったが、public contract と emitted artifact family は後段 |
| storage / backend guardrail | 72% | 82% | 76% | external workdir と probe floor はあるが、actual LLVM build はまだ |

## macro phase map

| Macro phase | 主眼 | 現在位置 | rough progress % | 自走可否 |
|---|---|---:|---:|---|
| `Macro 0` | repository memory / docs / traceability | active | 98% | 着手可能 |
| `Macro 1` | semantic kernel / invariant stabilization | late | 87% | 着手可能 |
| `Macro 2` | parser-free validation substrate | late | 88% | 着手可能 |
| `Macro 3` | compile-ready minimal actualization | late | 84% | public parser / API 以外は着手可能 |
| `Macro 4` | executable sample floor | active clean suite | 98% | 着手可能 |
| `Macro 5` | typed / theorem / model-check bridge | repo-local alpha-ready current layer | 96% | public seam 以外は自走可能 |
| `Macro 6` | shared-space / fabric boundary | helper/report-local preview widening 中 | 90% | public contract / real transport 以外は自走可能 |
| `Macro 7` | toolchain / backend / host integration | mounted workdir + guardrail | 72% | installed binary / backend choice 以外は段階的に自走可能 |
| `Macro 8` | application realization | early | 22% | 要仕様確認 |

## feature family snapshot

| feature family | 現在地 | できていること | まだ残ること |
|---|---|---|---|
| executable sample corpus | `S6` | active clean suite、Sugoroku、avatar representative slice、dashboard | final public sample catalog |
| finite-index typing / order-handoff | `S6` | authority / label / capture / region / cost finite theory、high-level relation family | final typed source principal と public checker / artifact contract |
| theorem / model-check / Lean | `S5-S6` | model-check second line、small Lean proof fragment、generated stub corpus | full domain discharge と concrete external tool binding |
| shared-space runtime samples | `S6` | attach / membership / handoff / follow / fallback / reset safety | detach lifecycle residual、real transport、durable evidence |
| typed external / projection / hot-plug / transport preview | `S5` | typed external synthetic preview、projection preview、hot-plug lifecycle canary、`NET-01..05` helper-local canary | final host-facing contract、emitted program family、real migration / replay |
| repository structure / dashboard | `S4-S5` | layer-aware repo map、sample/script taxonomy、front-door docs、snapshot docs | risky crate/path move をまだしていない |
| storage / backend guardrail | `S5` | external workdir、`target/` cutover、`CARGO_HOME` probe、LLVM path readiness | actual LLVM build、backend choice、packaging target |

## 着手可能 / 要仕様確認 / 後段依存

| 項目 | 状態 | 理由 |
|---|---|---|
| Typed external boundary residual planned family review | 着手可能 | current synthetic preview subset は actualize 済みで、residual `EXT-01` / `EXT-02` / `EXT-05` の reopen 条件を docs-first に整理できる |
| Projection / placement residual emitted-program gate | 後段依存 | preview floor は actualize 済みだが、emitted program / optimizer / equivalence line は mixed gate に残す |
| `FAIRY-05` residual reacquire widening | 後段依存 | explicit state timeline / anchor switch evidence gate は固定済み。carrier bundling は `UNRESOLVED` |
| `HotPlug Patch / AttachPoint` real migration | 後段依存 | helper-local lifecycle canary はあるが、migration / rollback contract は別 gate |
| final public auth / visualization / projection / hot-plug API | 要仕様確認 | public contract と retention / ecosystem target が未定 |
| packaging / FFI / broader application target | 要仕様確認 | distribution target と acceptance criteria が未定 |

## 再現性アンカー

- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `python3 scripts/current_l2_guided_samples.py closeout --format json`
- `python3 scripts/sugoroku_world_samples.py closeout --format json`
- `python3 scripts/avatar_follow_samples.py closeout --format json`
- `python3 scripts/typed_external_boundary_samples.py closeout --format json`
- `python3 scripts/network_transport_samples.py closeout --format json`
- `cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json`
- `git diff --check`

## recent log

- 2026-04-27 23:26 JST — `README.md` / `Documentation.md` / `progress.md` / `tasks.md` / `samples_progress.md` を snapshot として圧縮・同期し、current next queue、front-door docs、recent validation の読みを current line に揃えた。`0943` report を追加した上で `check_source_hierarchy.py`、`validate_docs.py`、typed external closeout、focused projection run、runtime sample run、`git diff --check` を通した。
- 2026-04-27 22:52 JST — `Projection / placement executable widening` として Sugoroku helper `projection_view` / `--debug projection` と clean near-end runtime report-local `cross_place_projection` を追加し、next promoted package を Typed external boundary residual planned family review に進めた。
- 2026-04-27 22:21 JST — phase 9 wording を recut し、`EXT-03` / `EXT-04` を planned source stub を読む synthetic preview helper subset と明記した。
- 2026-04-27 21:55 JST — `Typed external boundary executable widening` として `scripts/typed_external_boundary_samples.py`、synthetic preview subset `EXT-03` / `EXT-04`、residual planned family `EXT-01` / `EXT-02` / `EXT-05` を同期した。
- 2026-04-27 21:38 JST — `FAIRY-05 residual reacquire design review` として sample は planned のままに保ちつつ、explicit state timeline / anchor switch evidence gate と `UNRESOLVED` carrier bundling を docs-first に固定した。
- 2026-04-27 20:55 JST — `Avatar fairy follow residual widening` として `FAIRY-02` を active helper canary に昇格させ、residual planned family を `FAIRY-05` だけに縮めた。
- 2026-04-27 20:27 JST — `Network transport helper-local canaries` として `NET-02..05` を helper-local evidence surface に actualize した。
- 2026-04-27 20:03 JST — `HotPlug Patch / AttachPoint executable widening` として `hotplug_lifecycle`、`--debug hotplug`、attach / detach telemetry-view を current line に actualize した。
- 2026-04-27 19:46 JST — `Avatar fairy follow representative slice` として active avatar helper / sample root / debug surface を同期した。
