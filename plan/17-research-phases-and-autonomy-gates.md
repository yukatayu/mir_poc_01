# plan/17 — research phases and autonomy gates

## 目的

この文書は、repo 全体の長期研究を

- legacy checkpoint
- macro phase
- autonomy gate

の 3 つで見やすく整理する。

ここでいう phase は strict waterfall ではない。
複数の line が並走してよい。

## how to read this document

### legacy checkpoint

- `specs/examples/...` や report に出てくる `Phase 1..7` は、当時の closeout / freeze / reopen line を示す historical label である。
- これは source traceability のために残す。

### macro phase

- `progress.md` と `tasks.md` では、repo 全体の大局を **macro phase** で整理する。
- old `Phase 7 = FutureWork` を再導入しないためである。

## legacy checkpoint reading

| legacy checkpoint | current reading |
|---|---|
| Phase 1 | closeout fixed |
| Phase 2 | closeout fixed |
| Phase 3 | freeze fixed |
| Phase 4 | closeout fixed |
| Phase 5 | closeout fixed |
| legacy Phase 6 front-half | compile-ready checkpoint fixed |
| old Phase 7 | 今後は単独 bucket としては使わず、macro phase `6/7/8` へ分解する |

## macro phase 一覧

| Macro phase | 主眼 | 現在位置 | 重さ | autonomy gate |
|---|---|---|---|---|
| `Macro 0` | repository memory / docs / traceability | maintenance | 低い | self-driven |
| `Macro 1` | semantic kernel / invariant stabilization | late | 中 | self-driven |
| `Macro 2` | parser-free validation substrate | late | 中 | self-driven |
| `Macro 3` | compile-ready minimal actualization | late | 中〜重い | self-driven |
| `Macro 4` | executable fixed-subset sample expansion | active | 重い | self-driven |
| `Macro 5` | theorem / model-check / static reasoning bridge | early | 重い | self-driven up to boundary |
| `Macro 6` | fabric / shared-space / runtime evolution | docs-first boundary only | 重い | mixed |
| `Macro 7` | toolchain / backend / developer surface | inventory only | 重い | mixed |
| `Macro 8` | domain / application realization | not started | とても重い | user spec required |

## autonomy gate detail

### self-driven でよい line

- semantic kernel の narrow refinement
- parser-free regression / helper maintenance
- compile-ready minimal actualization の narrow widening
- fixed-subset source sample expansion
- theorem-side bridge の compare-ready / first pilot

### boundary までは self-driven でよい line

- Mirrorea / shared-space の docs-first boundary
- public operational surface の inventory

### user specification が必要な line

- shared-space final activation / authority / auth / identity / admission / consistency / fairness catalog
- upper-layer application target
- final backend / tooling success criteria

## current mainline

1. `Macro 4` actual `e3` authored-row reopen
2. `Macro 5` proof / model-check first concrete tool pilot
3. `Macro 4` second source-sample cluster sequencing
4. `Macro 3-7` parser / checker / runtime public surface inventory

## current stop lines

次はまだ finalization しない。

- final parser grammar
- final public parser / checker / runtime API
- concrete theorem / model-check production contract
- shared-space final catalog
- backend / codegen public surface
- upper-layer app contract

## current judgments

- current repo は architecture-first だが、fixed-subset runnable path を already 持つ。
- current mainline は `Macro 4` と `Macro 5` にある。
- Mirrorea / shared-space / backend / apps は old `FutureWork` ではなく separable track として扱う。
- current immediate mainline を止める user decision は 0 件である。
