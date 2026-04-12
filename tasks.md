# tasks

最終更新: 2026-04-12 21:52 JST

## この文書について

- この文書は repo 全体の **current task map** である。
- この snapshot では、`.docs/progress-task-axes.md` に従い、
  - **legacy checkpoint**
  - **macro phase**
  - **feature maturity stage**
  を分けて扱う。
- 規範判断の正本は `specs/`、長期比較と repository memory は `plan/`、詳細な経緯は `docs/reports/` に置く。
- `tasks.md` は append-only の履歴ではない。**毎回 current snapshot に合わせて全体を書き直す。**

## current checkpoint

- legacy checkpoint としては、Phase 1〜5 closeout / freeze は fixed 済みである。
- legacy Phase 6 front-half compile-ready checkpoint も fixed 済みであり、parser / checker-runtime / formal-hook / source-sample first packages / theorem-first review-unit pilot / plain bridge sketch first actualization / compare-ready bridge sketch second reopen までは current entry criteria と読んでよい。
- current macro phase としては、
  - `Macro 4` executable fixed-subset sample expansion
  - `Macro 5` theorem / verifier bridge
  が mainline である。
- current promoted line は **actual `e3` authored-row reopen** であり、その後に
  - proof / model-check first concrete tool pilot
  - second source-sample cluster sequencing
  - parser / checker / runtime public surface inventory
  を置くのが自然である。

## 次に自走で進める順番

| 順番 | macro phase | feature family | current stage -> next stage | task package | rough 所要 | 自走可否 |
|---|---|---|---|---|---|---|
| 1 | `Macro 4` | source-backed sample corpus | `S4 -> S5` | actual `e3` authored-row package | 1〜2 task / 数日 | 自走可能 |
| 2 | `Macro 5` | external verifier bridge | `S2-S3 -> S4` | proof / model-check first concrete tool pilot | 1〜3 task / 1〜2週 | 自走可能 |
| 3 | `Macro 4` | sample corpus widening | `S3-S4 -> S5` | second source-sample cluster sequencing after `e3` | 2〜4 task / 1〜2週 | 自走可能 |
| 4 | `Macro 3-7` | public operational surface | `S1-S2 -> S3` | parser / checker / runtime public surface inventory | 2〜4 task / 1〜2週 | 一部自走可能 |
| 5 | `Macro 6` | fabric / shared-space boundary | `S2-S3` 維持 | Mirrorea / shared-space docs-first re-entry package | 2〜5 task / 複数週 | boundary までは自走可能 |

## 自走可能な task package

### Package 1. actual `e3` authored-row package

- macro phase
  - `Macro 4`
- 目的
  - reopen timing fixed 後に `e3-option-admit-chain` を actual authored row に上げる最小 package を切る。
- 完了条件
  - theorem-side bridge line と formal-hook guard を壊さず、`e3` を current authored inventory と regression ladder に接続する。current formal-hook top widening は still later に残す。

### Package 2. proof / model-check first concrete tool pilot

- macro phase
  - `Macro 5`
- 目的
  - `e3` reopen の後段として、proof notebook / theorem / model-check sideの first concrete tool cut を narrow reopen する。
- 完了条件
  - tool-neutral formal hook と current bridge sketch cut を壊さず、first concrete consumer pressure を narrow pilot に留める。

### Package 3. second source-sample cluster sequencing after `e3`

- macro phase
  - `Macro 4`
- 目的
  - current authored cluster の次に、どの sample family を widen するかを順序づける。
- 完了条件
  - settled subset の sample expansion と heavy feature expansion を混ぜず、next cluster を docs / fixture / source / regression の 4 層で切る。

### Package 4. parser / checker / runtime public surface inventory

- macro phase
  - `Macro 3-7`
- 目的
  - current helper-local / non-production surface と later public operational surface の境界を inventory 化する。
- 完了条件
  - final public parser / checker / runtime API をまだ固定せず、どこまで helper-local に留めるかを docs-first に整理する。

### Package 5. Mirrorea / shared-space docs-first re-entry

- macro phase
  - `Macro 6`
- 目的
  - current mainline が一段落した後に、fabric / shared-space line を old `FutureWork` bucket ではなく独立 track として再開する。
- 完了条件
  - Mirrorea、shared-space、Typed-Effect、Prism、apps を 1 行に潰さず、boundary と user-spec gate を明示する。

## 研究を通して見つけること

### Discovery 1. `e3` authored-row の ladder shape

- 概要
  - actual `e3` authored-row package で、どこまで reached-stage row を上げ、どこから still guarded に残すか。
- 何に影響するか
  - current authored inventory
  - regression helper / README ladder / formal-hook guard wording
- current recommendation
  - **source row / runner / authored inventory / regression ladder を先に actualize し、current formal-hook top widening は still later に残す**。

### Discovery 2. source-sample second cluster の順序

- 概要
  - `e3` の後にどの feature family を runnable sample として厚くするか。
- 何に影響するか
  - executable subset の拡張速度
  - semantics drift の抑制
- current recommendation
  - **settled subset を厚くし、heavy async-control / memory-order family を混ぜない**。

### Discovery 3. concrete theorem / model-check bridge の最小 carrier

- 概要
  - tool-neutral formal hook の後で、どの concrete tool cut を first pilot に置くか。
- 何に影響するか
  - public checker migration
  - retained artifact / bless policy
- current recommendation
  - **proof notebook current line を起点に、first concrete cut を narrow に切る**。

### Discovery 4. backend / public operational surface の timing

- 概要
  - LLVM-family backend や public operational CLI をいつ mainline に入れるか。
- 何に影響するか
  - syntax / IR / public API の早期固定
- current recommendation
  - **source sample / theorem-side bridge のあと** に narrow inventory から開く。

## user が決める必要があること

### Decision 1. shared-space final catalog

- 概要
  - activation / authority / auth / identity / admission / consistency / fairness の final catalog。
- 何に影響するか
  - `Macro 6` 全体
  - upper-layer runtime / governance semantics
- 主要な選択肢
  - authoritative 寄り
  - append-friendly 寄り
  - hybrid / policy overlay
- current recommendation
  - **current repo では docs-first boundary までに留め、final catalog は user が決める**。

### Decision 2. upper-layer application target

- 概要
  - first-class に何を application goal とするか。
- 何に影響するか
  - `Macro 8`
  - sample / demo / acceptance criteria
- 主要な選択肢
  - synchronized shared-space
  - collaborative editing
  - virtual-world / avatar-oriented example
  - other domain target
- current recommendation
  - **current mainline を止める immediate blocker ではないが、Macro 8 へ進む前に必要**。

### Decision 3. backend / tooling success criteria

- 概要
  - backend / editor / operational CLI に何を first success criterion とするか。
- 何に影響するか
  - `Macro 7`
  - Rust/Python split の将来比率
- current recommendation
  - **今は inventory と narrow pilot だけに留め、final success criteria は later で user と合わせる**。

## 現在の blocker 読み

- **current Package 1〜5 を止める immediate blocker は 0 件** と読むのが自然である。
- current blocker は主に research-discovery 側にあり、user decision は主として `Macro 6` 以降の gate にある。
