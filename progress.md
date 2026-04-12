# progress

最終更新: 2026-04-12 21:52 JST

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
  - `e21`
  - `e4`
  - `e23`
  の quintet である。
- `e3` は source target only / deferred authored row のままであり、theorem-side line を先に比較する current guard を保っている。
- tool-neutral formal hook から `proof_notebook_review_unit` を作る first theorem-side pilot と、`comparison_basis_refs` までを持つ compare-ready bridge sketch second reopen、deferred `e3` actualization reopen timing はあるが、actual `e3` source row、bless / review-session metadata、concrete theorem / model-check binding はまだ later である。
- final parser grammar、final public parser / checker / runtime API、LLVM-family backend、Mirrorea operational runtime、shared-space final catalog はまだ無い。

## Macro phase map

| Macro phase | rough % | 主眼 | 現在位置 | 自走可否 | 補足 |
|---|---:|---|---|---|---|
| `Macro 0` | 95% | repository memory / docs / traceability | maintenance | 自走可能 | report / snapshot / plan / FAQ の整流ルールはかなり安定 |
| `Macro 1` | 94% | semantic kernel / invariant stabilization | late | 自走可能 | current L2 semantics / invariant bridge はかなり安定 |
| `Macro 2` | 97% | parser-free validation substrate | late | 自走可能 | parser-free PoC と detached validation loop は runnable |
| `Macro 3` | 90% | compile-ready minimal actualization | late | 自走可能 | parser / checker / runtime / formal-hook の non-production minimal cut は揃った |
| `Macro 4` | 61% | executable fixed-subset sample expansion | active | 自走可能 | current mainline。authored quintet runnable、`e3` actualization と second cluster widening が残る |
| `Macro 5` | 38% | static reasoning / theorem / model-check bridge | early | 一部自走可能 | tool-neutral formal hook、review-unit pilot、plain bridge sketch、compare-ready bridge sketch second reopen はあるが concrete tool binding は later |
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
| guarded option chain / `lease` / monotone degradation | `S5` | semantics / invariant / fixture baseline は厚い。parser-free 実行もある | full syntax / algebra、`Approximate` / `Compensate`、syntax-backed `e3` authored row | 自走可能 |
| parser-free validation loop | `S6` | interpreter / host harness / bundle / batch / detached compare loop が runnable | public exporter / retention policy / detached serialization | 自走可能 |
| parser / checker / runtime actualization | `S4-S5` | `mir-ast` / `mir-semantics` / `mir-runtime` の narrow non-production tranche が compile-ready | final grammar、public surface、richer host interface | 自走可能 |
| source-backed sample corpus / verification ladder | `S5` | current authored quintet `e1` / `e2` / `e21` / `e4` / `e23` が ladder に乗る | `e3` actualization、second cluster widening、final CLI | 自走可能 |
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
| fixed-subset source samples | 84% | 87% | 83% | active | authored quintet runnable、`e3` 以降は widen line |
| theorem / verifier bridge | 78% | 73% | 44% | early | formal hook と proof notebook first bridge はある |
| Mirrorea fabric boundary | 63% | 51% | 8% | docs-first | no-shadowing / overlay / patch discipline は整理済み |
| shared-space boundary | 68% | 58% | 12% | docs-first | practical cut はあるが final catalog は user spec required |
| Typed-Effect Wiring Platform | 34% | 26% | 4% | positioning only | subsystem boundary はあるが actual line は薄い |
| PrismCascade | 28% | 20% | 3% | positioning only | positioning / separation が主で、actual kernel work は later |
| domain / application layer | 12% | 9% | 1% | not started | target application と success criteria が未確定 |

## 現在の self-driven line

1. **Macro 4 / actual `e3` authored-row reopen**
   - reopen timing fixed 後に `e3-option-admit-chain` を actual authored row に上げる最小 package を切る。
2. **Macro 5 / proof-model-check first concrete tool pilot**
   - `e3` reopen 後に concrete theorem / model-check side の最小 handoff pilot を比較する。
3. **Macro 4 / second source-sample cluster sequencing**
   - `e3` close 後に next source-sample family を narrow に順序づける。
4. **Macro 3-7 / parser-checker-runtime public surface inventory**
   - helper-local current surface と later public operational surface の境界を narrow に inventory 化する。

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
- 2026-04-12 21:52 JST — `docs/reports/0649` と `specs/examples/343...344` で deferred `e3` actualization reopen timing を閉じ、`e3` authored-row line を compare-ready bridge の直後に reopen しつつ current formal-hook top wideningは still later に残す cut を固定した。current line は actual `e3` authored-row reopen に進んだ。
- 2026-04-12 21:44 JST — `docs/reports/0648` と `specs/examples/341...342` で compare-ready bridge sketch second reopen を閉じ、current theorem-side bridge を `comparison_basis_refs` まで持つ docs-only second actualization に固定した。current line は deferred `e3` actualization reopen timing に進んだ。
- 2026-04-12 20:43 JST — `docs/reports/0644` `0645` `0646` を起点に phase recut / feature maturity / Rust-Python split を再整理し、`progress.md` と `tasks.md` を macro phase + feature maturity stage 方式へ全面更新した。
- 2026-04-12 20:10 JST — `docs/reports/0643` で post-task document consistency audit を閉じ、plain bridge sketch actualization が still-open に見える stale wording を `tasks.md` と relevant `plan/` から除去した。current line は compare-ready bridge sketch second reopen のまま維持した。
- 2026-04-12 20:04 JST — `docs/reports/0642` と `specs/examples/339...340` で plain proof-notebook bridge sketch actualization を閉じ、old theorem-line `specs/examples/140` の docs-only bridge shape を current theorem-side first actualization として再利用する cut を固定した。
- 2026-04-12 19:49 JST — `docs/reports/0641` と `specs/examples/337...338` で third widened row `e3` theorem-side / formal-hook guard comparison を閉じ、`e3` を deferred row のまま保つ current guard を固定した。
- 2026-04-12 19:33 JST — `docs/reports/0640` と `specs/examples/335...336` で second widened authored row `e21` actualization を閉じ、`e21-try-atomic-cut-frontier` を current authored inventory に昇格した。
