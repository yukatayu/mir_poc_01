# 431 — current L2 public operational CLI packaging reserve note

## 目的

`specs/examples/423` までで fixed した
current-L2 scoped Rust shell over thin facade を前提に、

- current actual shell concern
- installed-binary reserve
- final command hierarchy reserve
- packaging success criteria gate

の境界を docs-first に整理する。

ここで fixed するのは
**current shell actualization と packaging reserve の cut**
であり、

- installed binary promotion
- final `mir` top-level hierarchy
- final public parser / checker / runtime API
- final host/input contract
- backend / tooling success criteria

は still later に残す。

## source-backed floor

- current actual shell concern は
  `mir-current-l2 run-source-sample <sample> --host-plan <path> --format pretty|json`
  に留める。
- delegated library entry / report は
  `run_current_l2_source_sample` / `CurrentL2SourceSampleRunReport`
  に据え置く。
- `run_current_l2_runtime_skeleton` / `CurrentL2RuntimeSkeletonReport`
  は later support cut に残す。
- lowering helper、path resolution helper、accepted-set hard-coding、
  inventory / regression helper、repo-local Python orchestrationは excluded bucket に残す。

## current reserve split

| bucket | current reading |
|---|---|
| actualized floor | current-L2 scoped Rust shell helper + example wrapper |
| mixed gate A | installed-binary promotion を current shell concern とどこまで同一視してよいか |
| mixed gate B | `mir-current-l2` と final `mir` top-level hierarchy の naming / subcommand split |
| later gate | final host/input contract、backend / tooling success criteria、maintenance shell と public shell の final split |

## current judgment

1. packaging reserve note の仕事は、
   **current shell actualization を installed binary と読み替えないこと**
   を明示することである。
2. current shell concern を later packaging reserve に橋渡しするとしても、
   principal delegated entry は引き続き `run_current_l2_source_sample` に置く。
3. installed-binary promotion と final command hierarchy は別 gate に残す。
   前者は packaging / distribution concern、
   後者は final public shell family concern である。
4. current shell concern に maintenance verb、inventory verb、regression verb、
   runtime-skeleton support verb を追加しない。

## guard

- current shell actualization を installed binary fact と書かない。
- final `mir` top-level hierarchy を fixed しない。
- final public parser / checker / runtime API を fixed しない。
- final host/input contract を fixed しない。
- backend / tooling success criteria を fixed しない。

## next promoted line

next promoted line は、
**shared-space fairness / replay strengthening reserve note**
に置く。

## what is not decided here

- installed binary promotion の timing
- installed binary の final naming
- final `mir` top-level hierarchy
- maintenance shell と public shell の final split
- backend / tooling success criteria
- final host/input contract
