# tasks

最終更新: 2026-04-13 00:21 JST

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
- actual `e22` contrast-row source actualization も `specs/examples/351...352` により fixed 済みであり、current authored source sample は `e1` / `e2` / `e3` / `e21` / `e22` / `e4` / `e23` の septet に進んだ。
- stable static malformed post-contrast sequencing も `specs/examples/353...354` により fixed 済みであり、second broader cluster は stable reason-code / fixture-static cluster、Macro 4 side の next reopen point は `e4` / `e19` edge-pair に置く。
- parser / checker / runtime public surface inventory も `specs/examples/355...356` により fixed 済みであり、already-public parser-free stack、crate-public but non-production tranche、repo-local helper surface の 3 bucket split に整理した。
- current promoted line は **model-check / public-checker second reserve inventory** であり、その後に
  - stable-static edge-pair first reopen
  - public operational surface actualization gate
  - shared-space identity / auth layering reopen
  を置くのが自然である。

## 次に自走で進める順番

| 順番 | macro phase | feature family | current stage -> next stage | task package | rough 所要 | 自走可否 |
|---|---|---|---|---|---|---|
| 1 | `Macro 5` | external verifier bridge | `S3-S4 -> S4-S5` | model-check / public-checker second reserve inventory | 1〜3 task / 1〜2週 | 一部自走可能 |
| 2 | `Macro 4` | sample corpus widening | `S3-S4 -> S4-S5` | stable-static edge-pair first reopen | 1〜3 task / 数日〜1週 | 自走可能 |
| 3 | `Macro 3-7` | public operational surface | `S2-S3 -> S3-S4` | public operational surface actualization gate | 2〜4 task / 1〜2週 | 一部自走可能 |
| 4 | `Macro 6` | shared-space boundary | `S3 -> S3-S4` | shared-space identity / auth layering reopen | 1〜3 task / 数日〜1週 | boundary までは自走可能 |

## 自走可能な task package

### Package 1. model-check / public-checker second reserve inventory

- macro phase
  - `Macro 5`
- 目的
  - proof notebook first concrete cut の後で、model-check side / public-checker side をどの境界から reopen するかを inventory 化する。
- 完了条件
  - proof notebook current cut を巻き戻さず、machine-facing carrier の reserve line だけを narrow に整理する。

### Package 2. stable-static edge-pair first reopen

- macro phase
  - `Macro 4`
- 目的
  - stable static malformed broader cluster の first concrete reopen を `e4` / `e19` edge-pair sideへ narrow に actualize する。
- 完了条件
  - duplicate cluster と try/rollback malformed-static family を still later に残したまま、stable-static edge-pair line だけを source / fixture / ladder へ戻す。

### Package 3. public operational surface actualization gate

- macro phase
  - `Macro 3-7`
- 目的
  - public surface inventory fixed 後に、already-public parser-free stack を壊さずにどの current tranche から actual promotion pressure を受けるかを整理する。
- 完了条件
  - `pub visibility != final public contract` を保ったまま、later public API / CLI reopen の first sub-cut を narrow に示す。

### Package 4. shared-space identity / auth layering reopen

- macro phase
  - `Macro 6`
- 目的
  - Mirrorea/shared-space docs-first re-entry fixed 後の next docs-first reopen として、membership core と auth / admission / projection layering の cut を narrow に整理する。
- 完了条件
  - principal identity、transport / service auth、room admission、display / projection identity を membership carrier に潰さない current boundary を current task map に昇格する。

## 研究を通して見つけること

### Discovery 1. stable-static edge-pair first reopen の bundle shape

- 概要
  - `e4` / `e19` を stable-static edge-pair first reopen に戻すとき、source / fixture / ladder / formal-hook のどこまでを同時に太らせるか。
- 何に影響するか
  - executable subset の widening speed
  - static malformed cluster の drift suppression
- current recommendation
  - **stable reason-code / fixture-static cluster selection は fixed とし、first reopen は `e4` / `e19` edge-pair sideへ narrow に戻す**。

### Discovery 2. public operational surface の actualization timing

- 概要
  - inventory fixed 後に、parser / checker / runtime current tranche のどこから actual public pressure を受けるか。
- 何に影響するか
  - crate boundary
  - future CLI / public API timing
- current recommendation
  - **parser-free existing public behavior を維持し、current tranche の actual promotion は still later の first sub-cut に残す**。

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
