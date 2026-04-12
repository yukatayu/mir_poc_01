# tasks

最終更新: 2026-04-12 22:35 JST

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
- legacy Phase 6 front-half compile-ready checkpoint も fixed 済みであり、parser / checker-runtime / formal-hook / source-sample first packages / theorem-first review-unit pilot / plain bridge sketch first actualization / compare-ready bridge sketch second reopen / actual `e3` authored-row actualization までは current entry criteria と読んでよい。
- current macro phase としては、
  - `Macro 4` executable fixed-subset sample expansion
  - `Macro 5` theorem / verifier bridge
  が mainline である。
- second source-sample cluster sequencing は `specs/examples/349...350` により fixed 済みであり、post-sextet first cluster は `e21` / `e22` try-rollback locality contrast に置く。
- current promoted line は **actual `e22` contrast-row source actualization** であり、その後に
  - stable static malformed post-contrast sequencing
  - parser / checker / runtime public surface inventory
  - Mirrorea / shared-space docs-first re-entry
  - model-check / public-checker second reserve inventory
  を置くのが自然である。

## 次に自走で進める順番

| 順番 | macro phase | feature family | current stage -> next stage | task package | rough 所要 | 自走可否 |
|---|---|---|---|---|---|---|
| 1 | `Macro 4` | sample corpus widening | `S4 -> S5-S6` | actual `e22` contrast-row source actualization | 1〜3 task / 数日〜1週 | 自走可能 |
| 2 | `Macro 4` | sample corpus widening | `S3-S4 -> S4-S5` | stable static malformed post-contrast sequencing | 1〜3 task / 数日〜1週 | 自走可能 |
| 3 | `Macro 3-7` | public operational surface | `S1-S2 -> S3` | parser / checker / runtime public surface inventory | 2〜4 task / 1〜2週 | 一部自走可能 |
| 4 | `Macro 6` | fabric / shared-space boundary | `S2-S3` 維持 | Mirrorea / shared-space docs-first re-entry package | 2〜5 task / 複数週 | boundary までは自走可能 |
| 5 | `Macro 5` | external verifier bridge | `S3-S4 -> S4-S5` | model-check / public-checker second reserve inventory | 1〜3 task / 1〜2週 | 一部自走可能 |

## 自走可能な task package

### Package 1. actual `e22` contrast-row source actualization

- macro phase
  - `Macro 4`
- 目的
  - `e21` / `e22` try-rollback locality contrast を first post-sextet cluster として actual source row まで接続する。
- 完了条件
  - `e22` source text / runner accepted set / regression inventory / README ladder / formal-hook smoke route を current contrast row として同期する。

### Package 2. stable static malformed post-contrast sequencing

- macro phase
  - `Macro 4`
- 目的
  - `e22` contrast close 後の broader follow-up cluster を stable static malformed family のどこから reopen するかを narrow に決める。
- 完了条件
  - duplicate cluster と stable reason-code cluster を混ぜず、next broader cluster を docs / fixture / regression の 3 層で切る。

### Package 3. parser / checker / runtime public surface inventory

- macro phase
  - `Macro 3-7`
- 目的
  - current helper-local / non-production surface と later public operational surface の境界を inventory 化する。
- 完了条件
  - final public parser / checker / runtime API をまだ固定せず、どこまで helper-local に留めるかを docs-first に整理する。

### Package 4. Mirrorea / shared-space docs-first re-entry

- macro phase
  - `Macro 6`
- 目的
  - current mainline が一段落した後に、fabric / shared-space line を old `FutureWork` bucket ではなく独立 track として再開する。
- 完了条件
  - Mirrorea、shared-space、Typed-Effect、Prism、apps を 1 行に潰さず、boundary と user-spec gate を明示する。

### Package 5. model-check / public-checker second reserve inventory

- macro phase
  - `Macro 5`
- 目的
  - proof notebook first concrete cut の後で、model-check side / public-checker side をどの境界から reopen するかを inventory 化する。
- 完了条件
  - proof notebook current cut を巻き戻さず、machine-facing carrier の reserve line だけを narrow に整理する。

## 研究を通して見つけること

### Discovery 1. post-contrast stable static malformed subcluster の順序

- 概要
  - `e22` contrast close の後で、stable static malformed family のどこを second broader cluster に置くか。
- 何に影響するか
  - executable subset の拡張速度
  - semantics drift の抑制
- current recommendation
  - **stable reason-code / fixture-static cluster を優先し、duplicate family や broader runtime family を混ぜない**。

### Discovery 2. public operational surface の inventory timing

- 概要
  - parser / checker / runtime current helper-local surface をどの順で public inventory に切り出すか。
- 何に影響するか
  - crate boundary
  - future CLI / public API timing
- current recommendation
  - **current helper-local surface を先に inventory 化し、final public API は still later に残す**。

### Discovery 3. model-check / public-checker second reserve の入口

- 概要
  - proof notebook の後で、model-check side や public checker line をどの順で reopen するか。
- 何に影響するか
  - public checker migration
  - retained artifact / bless policy
- current recommendation
  - **proof notebook current line を first pilot に維持し、model-check side と public checker migration はその後段に押し分ける**。

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
