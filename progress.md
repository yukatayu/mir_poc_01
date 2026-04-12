# progress

最終更新: 2026-04-12 22:21 JST

## この文書について

- この文書は repo 全体の **簡潔な進捗スナップショット** である。
- 規範判断の正本は `specs/`、長期比較と repository memory は `plan/`、詳細な経緯は `docs/reports/` に置く。
- ここに書く進捗率は **rough estimate** であり、問題が見つかれば巻き戻る。
- 高い進捗率は、特に断りがない限り **current-L2 fixed-subset / non-production line に scoped した reading** である。
- この文書では、old `Phase 1..7` checkpoint label に加えて、`.docs/progress-task-axes.md` で定義した
  - **macro phase**
  - **feature maturity stage**
  の 2 軸で大局を整理する。

## 現在の capability snapshot

- legacy checkpoint としては、Phase 1〜5 closeout / freeze は fixed 済みである。
- legacy Phase 6 front-half compile-ready checkpoint も fixed 済みであり、`mir-ast` / `mir-semantics` / `mir-runtime` の narrow actual path、tool-neutral formal hook、proof-notebook first pilot、plain bridge sketch first actualization、compare-ready bridge sketch second reopen までは source-backed である。
- parser-free current L2 PoC は runnable であり、compile / test / detached-loop baseline がある。
- syntax-backed fixed-subset sample path も runnable であり、現在 authored 済みなのは
  - `e1`
  - `e2`
  - `e3`
  - `e21`
  - `e4`
  - `e23`
  の sextet である。
- `e3` は source row / runner / inventory / ladder まで actualize 済みであり、`admit-miss` を non-admissible skip に留めた runtime success まで reached している。一方で current theorem-side consumer `proof_notebook_review_unit` と current formal-hook top `runtime_try_cut_cluster` はそのまま保ち、formal hook stage は `not reached (guarded)` に留めている。
- tool-neutral formal hook から `proof_notebook_review_unit` を作る first theorem-side pilot と、`comparison_basis_refs` までを持つ compare-ready bridge sketch second reopen はあるが、bless / review-session metadata、concrete theorem / model-check binding はまだ later である。
- final parser grammar、final public parser / checker / runtime API、LLVM-family backend、Mirrorea operational runtime、shared-space final catalog はまだ無い。

## Macro phase map

| Macro phase | rough % | 主眼 | 現在位置 | 自走可否 | 補足 |
|---|---:|---|---|---|---|
| `Macro 0` | 95% | repository memory / docs / traceability | maintenance | 自走可能 | report / snapshot / plan / FAQ の整流ルールはかなり安定 |
| `Macro 1` | 94% | semantic kernel / invariant stabilization | late | 自走可能 | current L2 semantics / invariant bridge はかなり安定 |
| `Macro 2` | 97% | parser-free validation substrate | late | 自走可能 | parser-free PoC と detached validation loop は runnable |
| `Macro 3` | 90% | compile-ready minimal actualization | late | 自走可能 | parser / checker / runtime / formal-hook の non-production minimal cut は揃った |
| `Macro 4` | 69% | executable fixed-subset sample expansion | active | 自走可能 | current authored sextet runnable。`e3` は formal-hook guarded 付きで ladder に乗ったので、次は second cluster sequencing |
| `Macro 5` | 45% | static reasoning / theorem / model-check bridge | early | 一部自走可能 | proof notebook review-unit current cut を first concrete pilot に固定。model-check side は second reserve |
| `Macro 6` | 22% | distributed fabric / shared-space / runtime evolution | docs-first boundary only | 境界までは自走可能 | Mirrorea / shared-space は boundary と practical cut が先。final catalog は user spec required |
| `Macro 7` | 14% | toolchain / backend / developer surface | inventory only | 一部自走可能 | Rust-heavy direction はあるが、backend / public operational surface はまだ早い |
| `Macro 8` | 5% | domain / application realization | not started | 要仕様確認 | 上位アプリの concrete goal が未確定 |

## Feature maturity stage legend

| Stage | 意味 |
|---|---|
| `S0` | 要求探索 |
| `S1` | 理論骨格 |
| `S2` | 論理学 / 計算機科学 / invariant 整理 |
| `S3` | current spec / boundary convergence |
| `S4` | narrow implementation |
| `S5` | executable / validation ratchet |
| `S6` | sample / human docs integration |

## 特徴機能ごとの進捗

| Feature family | current stage | 現在できていること | 主な残件 | 進め方 |
|---|---|---|---|---|
| `event DAG` / `place` / local `try` / `atomic_cut` | `S5-S6` | fixed subset では runnable。parser-free fixture と source sample の双方で evidence がある | `durable_cut`、distributed finalization、final syntax | 自走可能 |
| guarded option chain / `lease` / monotone degradation | `S5-S6` | semantics / invariant / fixture baseline は厚く、`e3` source-authored row まで runtime success で通る | full syntax / algebra、`Approximate` / `Compensate`、formal-hook widening | 自走可能 |
| parser-free validation loop | `S6` | interpreter / host harness / bundle / batch / detached compare loop が runnable | public exporter / retention policy / detached serialization | 自走可能 |
| parser / checker / runtime actualization | `S4-S5` | `mir-ast` / `mir-semantics` / `mir-runtime` の narrow non-production tranche が compile-ready | final grammar、public surface、richer host interface | 自走可能 |
| source-backed sample corpus / verification ladder | `S5-S6` | current authored sextet `e1` / `e2` / `e3` / `e21` / `e4` / `e23` が ladder に乗る。`e3` は formal hook だけ guarded | second cluster widening、final CLI、public retention path | 自走可能 |
| contracts / static gate / formal hook / proof notebook first bridge | `S4-S5` | tool-neutral formal hook、review-unit pilot、plain bridge sketch first actualization、compare-ready bridge sketch second reopenがある | bless / review-session metadata、concrete theorem / model-check binding、public checker line | 自走可能 |
| multi-node / routing / overlay / safe downstream addition | `S2-S3` | semantic constraintsと docs-first boundaryはある | route proof、suspended task / patch interaction、operational realization | 境界までは自走可能 |
| shared-space / membership / authority / fairness | `S3` for boundary, `S1-S2` for runtime realization | authoritative room baseline と control-plane threshold はある | final activation / authority / auth / identity / admission / consistency / fairness catalog | user spec required beyond boundary |
| type system / theorem prover / model checker full line | `S1-S2` | boundary inventory と first theorem-side pilot はある | strong type system、concrete prover/model-check binding、final external contract | heavy self-driven + later user goals |
| async-control / memory-order reconstruction | `S1` | `atomic_cut` local cut は source-backed | higher-level ordering / fairness / memory-order-like surface | heavy future research |
| backend / public dev surface / operational CLI | `S0-S1` | Rust-heavy core direction と some thin examples はある | backend IR bridge、public operational CLI、LSP / editor surface | later inventory |

## 層 / subsystem ごとの現在地

| Layer / subsystem | 論理仕様 | ユーザ向け仕様 | 実装 / 運用 | 現在位置 | 補足 |
|---|---:|---:|---:|---|---|
| Mir core semantics | 93% | 89% | 81% | late | current L2 semantic floor はかなり安定 |
| parser-free PoC / detached loop | 95% | 91% | 99% | late | runnable baseline と regression policy がある |
| compile-ready parser / checker / runtime | 87% | 76% | 84% | late | non-production minimal tranche は揃った |
| fixed-subset source samples | 88% | 90% | 88% | active | authored sextet runnable。next questionは second cluster widening |
| theorem / verifier bridge | 81% | 76% | 47% | early | formal hook と proof notebook first bridge はある。proof notebook current cut を first concrete pilot に固定し、`e3` は guarded non-reached row として接続済み |
| Mirrorea fabric boundary | 63% | 51% | 8% | docs-first | no-shadowing / overlay / patch discipline は整理済み |
| shared-space boundary | 68% | 58% | 12% | docs-first | practical cut はあるが final catalog は user spec required |
| Typed-Effect Wiring Platform | 34% | 26% | 4% | positioning only | subsystem boundary はあるが actual line は薄い |
| PrismCascade | 28% | 20% | 3% | positioning only | positioning / separation が主で、actual kernel work は later |
| domain / application layer | 12% | 9% | 1% | not started | target application と success criteria が未確定 |

## 現在の self-driven line

1. **Macro 4 / second source-sample cluster sequencing**
   - `e3` close 後に next source-sample family を narrow に順序づける。
2. **Macro 3-7 / parser-checker-runtime public surface inventory**
   - helper-local current surface と later public operational surface の境界を narrow に inventory 化する。
3. **Macro 6 / Mirrorea shared-space docs-first re-entry**
   - old FutureWork bucket に押し込めず、fabric / shared-space line を独立 track として再開する。
4. **Macro 5 / model-check public-checker second reserve inventory**
   - proof notebook current cut の後で machine-facing carrier をどの境界から reopen するかを整理する。

## 研究で見つけることと、user が決めること

### 研究で見つけること

- fixed-subset source-sample second cluster の widen 順序
- concrete theorem / model-check bridge の最小 carrier
- backend / public operational surface をどこで narrow pilot するか

### user が決めること

- shared-space final activation / authority / auth / identity / admission / consistency / fairness catalog
- upper-layer application の concrete goal と success criteria
- domain example を何から first-class に扱うか

### current reading

- current immediate mainline を止める **user decision は 0 件** と読んでよい。
- current blocker は主に research-discovery 側であり、shared-space / app 側の user decision は later gate にある。

## 実装言語の current split

- **Rust**
  - parser carrier
  - parser-free interpreter
  - static gate
  - runtime thin bridge
  - source lowerer / sample runner
  - formal hook / review-unit emitters
- **Python**
  - docs validation
  - report scaffolding
  - detached-loop orchestration
  - source-sample regression orchestration
  - compare/checker helper

current reading は、**Rust-heavy core + mixed-tool helper workflow** である。
「repo 全体を最終的に全部 Rust へ寄せる」とまでは、現時点の docs からは読まない。

## recent log

- 注記: この欄は recent log として保つ。詳細な履歴は `docs/reports/` を正本にする。
- 2026-04-12 22:21 JST — `docs/reports/0651` と `specs/examples/347...348` で proof / model-check first concrete tool pilot を閉じ、current first concrete carrier は row-local `proof_notebook_review_unit`、model-check side は second reserve に残す cut を再固定した。current line は second source-sample cluster sequencing に進んだ。
- 2026-04-12 22:09 JST — `docs/reports/0650` と `specs/examples/345...346` で actual `e3` authored-row reopen を閉じ、`e3-option-admit-chain` を source-authored row、runner accepted set、regression inventory、verification ladder へ actualize しつつ、formal hook は `not reached (guarded)` に留める cut を固定した。current line は proof / model-check first concrete tool pilot に進んだ。
- 2026-04-12 21:52 JST — `docs/reports/0649` と `specs/examples/343...344` で deferred `e3` actualization reopen timing を閉じ、`e3` authored-row line を compare-ready bridge の直後に reopen しつつ current formal-hook top wideningは still later に残す cut を固定した。current line は actual `e3` authored-row reopen に進んだ。
- 2026-04-12 21:44 JST — `docs/reports/0648` と `specs/examples/341...342` で compare-ready bridge sketch second reopen を閉じ、current theorem-side bridge を `comparison_basis_refs` まで持つ docs-only second actualization に固定した。current line は deferred `e3` actualization reopen timing に進んだ。
- 2026-04-12 20:43 JST — `docs/reports/0644` `0645` `0646` を起点に phase recut / feature maturity / Rust-Python split を再整理し、`progress.md` と `tasks.md` を macro phase + feature maturity stage 方式へ全面更新した。
- 2026-04-12 20:10 JST — `docs/reports/0643` で post-task document consistency audit を閉じ、plain bridge sketch actualization が still-open に見える stale wording を `tasks.md` と relevant `plan/` から除去した。current line は compare-ready bridge sketch second reopen のまま維持した。
- 2026-04-12 20:04 JST — `docs/reports/0642` と `specs/examples/339...340` で plain proof-notebook bridge sketch actualization を閉じ、old theorem-line `specs/examples/140` の docs-only bridge shape を current theorem-side first actualization として再利用する cut を固定した。
- 2026-04-12 19:49 JST — `docs/reports/0641` と `specs/examples/337...338` で third widened row `e3` theorem-side / formal-hook guard comparison を閉じ、`e3` を deferred row のまま保つ current guard を固定した。
- 2026-04-12 19:33 JST — `docs/reports/0640` と `specs/examples/335...336` で second widened authored row `e21` actualization を閉じ、`e21-try-atomic-cut-frontier` を current authored inventory に昇格した。
