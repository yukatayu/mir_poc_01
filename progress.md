# progress

最終更新: 2026-04-28 03:27 JST

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
- integration floor:
  `P0` current-state audit と `P1` repository layer map / `samples_progress.md` stabilization を close し、handoff 由来の queue numbering と source hierarchy 読みを current repo へ mirror した
- current promoted next line:
  `P2` Typed external boundary residual planned family review
- next reopen point:
  `P3` Projection / placement residual emitted-program gate
- still later:
  `P4-P9` carrier / representative-slice hardening、`P10-P17` first real implementation tranche、`P18` public API / parser grammar gate
- architectural caution:
  effect-based OS-like substrate は内側の解釈に留め、Mir core standard I/O primitive や subsystem collapse を既成事実化しない
- verification caution:
  `VerificationLayer` composition は typed layer composition の current reading に留め、hidden verifier builtin や final public verifier contract と混同しない

## 現在の一言での読み

2026-04-28 時点の repo は、**current-L2 / shared-space sample の runnable floor を維持したまま、Mirrorea future-axis の future-plan integration と next package queue stabilization を close し、次に `P2` typed external residual review と `P3` projection emitted-program gate を整理する段階**です。

## 3 軸 progress

| layer / track | 論理仕様 | ユーザ向け仕様 | 実装 / 運用 | 現在の読み |
|---|---:|---:|---:|---|
| Mir core / current-L2 | 90% | 88% | 80% | finite-index current layer は強いが、final parser / public API は未完 |
| order / handoff / cut family | 90% | 90% | 80% | high-level relation line は runnable。final source wording と public artifact contract は残る |
| theorem / model-check boundary | 92% | 90% | 86% | repo-local bridge は強いが、concrete external binding は未完 |
| Lean foundations / proof spine | 86% | 88% | 68% | small proof fragment と generated stub はあるが、full discharge ではない |
| shared-space samples | 84% | 87% | 75% | Sugoroku / avatar の runnable floor はあるが、real transport / durable evidence は未着手 |
| docs / dashboard / repository structure | 93% | 94% | 86% | source hierarchy、queue numbering、taxonomy、dashboard semantics は current line に追随 |
| Mirrorea future axis | 82% | 91% | 70% | helper-local / report-local preview は揃ったが、public contract と emitted artifact family は後段 |
| storage / backend guardrail | 72% | 83% | 76% | external workdir と probe floor はあるが、actual LLVM build はまだ |

## macro phase map

| Macro phase | 主眼 | 現在位置 | rough progress % | 自走可否 |
|---|---|---:|---:|---|
| `Macro 0` | repository memory / docs / traceability | active | 99% | 着手可能 |
| `Macro 1` | semantic kernel / invariant stabilization | late | 87% | 着手可能 |
| `Macro 2` | parser-free validation substrate | late | 88% | 着手可能 |
| `Macro 3` | compile-ready minimal actualization | late | 84% | public parser / API 以外は着手可能 |
| `Macro 4` | executable sample floor | active clean suite | 98% | 着手可能 |
| `Macro 5` | typed / theorem / model-check bridge | repo-local alpha-ready current layer | 96% | public seam 以外は自走可能 |
| `Macro 6` | shared-space / fabric boundary | `P2` / `P3` reserve gate待ち | 91% | public contract / real transport 以外は自走可能 |
| `Macro 7` | toolchain / backend / host-facing integration | guardrail + implementation tranche待ち | 72% | installed binary / backend choice 以外は段階的に自走可能 |
| `Macro 8` | application realization | early | 22% | 要仕様確認 |

## feature family snapshot

| feature family | 現在地 | できていること | まだ残ること |
|---|---|---|---|
| executable sample corpus | `S6` | active clean suite、Sugoroku、avatar representative slice、dashboard | final public sample catalog |
| finite-index typing / order-handoff | `S6` | authority / label / capture / region / cost finite theory、high-level relation family | final typed source principal と public checker / artifact contract |
| theorem / model-check / Lean | `S5-S6` | model-check second line、small Lean proof fragment、generated stub corpus | full domain discharge と concrete external tool binding |
| shared-space runtime samples | `S6` | attach / membership / handoff / follow / fallback / reset safety | detach lifecycle residual、real transport、durable evidence |
| typed external / projection / hot-plug / transport preview | `S5` | typed external synthetic preview、projection preview、hot-plug lifecycle canary、`NET-01..05` helper-local canary | final host-facing contract、emitted program family、real migration / replay |
| verification / visualization composition | `S4-S5` | `TermSignature`、`LayerSignature`、report-local inventories、typed visualization / telemetry first cut | exact `VerificationLayer` law surface、public viewer / verifier contract |
| repository structure / dashboard | `S6` | layer-aware repo map、sample/script taxonomy、front-door docs、snapshot docs | risky crate/path move をまだしていない |
| storage / backend guardrail | `S5` | external workdir、`target/` cutover、`CARGO_HOME` probe、LLVM path readiness | actual LLVM build、backend choice、packaging target |

## next package queue

| Package | Status | Objective | Stop line |
|---|---|---|---|
| `P0` current-state audit | closed | source hierarchy / stale reference / docs drift を補正する | risky move や public freeze を audit task に混ぜない |
| `P1` layer map / samples dashboard stabilization | closed | taxonomy と dashboard semantics を揃える | planned/helper/final-public の混同を再導入しない |
| `P2` typed external residual review | next | `EXT-01/02/05` residual planned family の reopen criterion を整理する | final host schema / adapter API を固定しない |
| `P3` projection emitted-program gate | reopen next | preview floor と emitted program family を切り分ける | final IR / optimizer / equivalence checker を固定しない |
| `P4-P7` carrier hardening | queued | signature / layer / envelope / visualization security を tighten する | helper-local inventory を public contract と誤読させない |
| `P8-P9` representative slice hardening | queued | Sugoroku / avatar residual gate を tighten する | real transport / final avatar API を claim しない |
| `P10-P17` first real implementation tranche | later | placeholder / preview floor から実装 tranche へ進める | subsystem collapse や premature freeze をしない |
| `P18` public API / parser grammar gate | final mixed gate | final freeze 条件を定義する | prior packages 未成熟のまま public freeze しない |

## 着手可能 / 要仕様確認 / 後段依存

| 項目 | 状態 | 理由 |
|---|---|---|
| `P2` Typed external boundary residual planned family review | 着手可能 | current synthetic preview subset は actualize 済みで、residual `EXT-01` / `EXT-02` / `EXT-05` の reopen 条件を docs-first に整理できる |
| `P3` Projection / placement residual emitted-program gate | 後段依存 | preview floor は actualize 済みだが、emitted program / optimizer / equivalence line は mixed gate に残す |
| `P4-P7` carrier hardening | 着手可能 | first cut はあるため、naming / law surface / stop line hardening を進められる |
| `P8-P9` representative slice hardening | 着手可能 | active representative slices はあり、residual gate を tightening できる |
| `P10-P17` first real implementation tranche | 後段依存 | docs-first / helper-local preview が先行しているため、実装 tranche は stop line を保ちながら段階的に進める必要がある |
| `P18` public API / parser grammar gate | 要仕様確認 | prior tranche の成熟と user 側の公開範囲判断が必要 |

## 再現性アンカー

- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `python3 scripts/current_l2_guided_samples.py closeout --format json`
- `python3 scripts/sugoroku_world_samples.py closeout --format json`
- `python3 scripts/avatar_follow_samples.py closeout --format json`
- `python3 scripts/typed_external_boundary_samples.py closeout --format json`
- `python3 scripts/network_transport_samples.py closeout --format json`
- `cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json`
- `cargo test -p mir-ast`
- `cargo test -p mir-runtime`
- `cargo test -p mir-semantics`
- `git diff --check`

## recent log

- 2026-04-28 03:27 JST — `sub-agent-pro/mirrorea_next_stage_full_plan_handoff_2026-04-27.md` を repo current line に mirror する task を close し、front-door docs、`AGENTS.md`、`specs/10` / `specs/11`、relevant `plan/`、`progress.md`、`tasks.md`、`samples_progress.md`、report `0945` を再同期した。`check_source_hierarchy.py`、`validate_docs.py`、clean suite smoke/closeout、Sugoroku / avatar / typed external / transport closeout、`cargo test -p mir-{ast,runtime,semantics}`、`git diff --check` を通した。
- 2026-04-27 23:26 JST — `README.md` / `Documentation.md` / `progress.md` / `tasks.md` / `samples_progress.md` を snapshot として圧縮・同期し、current next queue、front-door docs、recent validation の読みを current line に揃えた。`0943` report を追加した上で `check_source_hierarchy.py`、`validate_docs.py`、typed external closeout、focused projection run、runtime sample run、`git diff --check` を通した。
- 2026-04-27 22:52 JST — `Projection / placement executable widening` として Sugoroku helper `projection_view` / `--debug projection` と clean near-end runtime report-local `cross_place_projection` を追加し、next promoted package を Typed external boundary residual planned family review に進めた。
- 2026-04-27 21:55 JST — `Typed external boundary executable widening` として `scripts/typed_external_boundary_samples.py`、synthetic preview subset `EXT-03` / `EXT-04`、residual planned family `EXT-01` / `EXT-02` / `EXT-05` を同期した。
