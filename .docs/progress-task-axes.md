# Progress and Task Axes

## Purpose

この文書は、`progress.md` と `tasks.md` を更新するときの
repo-local な **大局整理の軸** を固定する。

重要なのは次の 3 点である。

1. 既存の `specs/examples/...` や report に出てくる `Phase 1..7` は、
   その時点の checkpoint / closeout line を示す **歴史的ラベル** として残る。
2. `progress.md` と `tasks.md` では、それとは別に
   **macro phase** と **feature maturity stage** の 2 軸を使う。
3. old `Phase 7 = FutureWork` のような巨大なまとめ bucket を再導入しない。

## Two planning axes

### Axis A — Macro phases

`progress.md` と `tasks.md` では、repo 全体の大局を次の macro phase で整理する。

| Macro phase | 主眼 |
|---|---|
| `Macro 0` | repository memory / docs / traceability / maintenance discipline |
| `Macro 1` | semantic kernel / invariant / boundary stabilization |
| `Macro 2` | parser-free validation substrate / detached loop / regression baseline |
| `Macro 3` | compile-ready minimal actualization (`mir-ast` / `mir-semantics` / `mir-runtime`) |
| `Macro 4` | executable fixed-subset sample expansion / widening control |
| `Macro 5` | static reasoning / theorem / model-check / external verifier bridge |
| `Macro 6` | distributed fabric / shared-space / runtime evolution boundary |
| `Macro 7` | toolchain / backend / developer surface / public operational interface |
| `Macro 8` | domain / application realization |

### Axis B — Feature maturity stages

各 feature family は、必要に応じて次の maturity stage で表す。

| Stage | 意味 |
|---|---|
| `S0` | 要求探索前。何を解くかをまだ絞っている段階 |
| `S1` | 理論骨格。用語・境界・主要論点は見え始めたが、まだ骨格だけ |
| `S2` | 論理学 / 計算機科学 / invariant の整理段階 |
| `S3` | current spec / boundary convergence。docs-first で current cut が見えている |
| `S4` | narrow implementation。helper-local / non-production を含む最小 actualization がある |
| `S5` | executable / validation ratchet。test / smoke / regression が通る経路がある |
| `S6` | sample / human docs integration。FAQ / README / progress / examples と同期されている |

## Snapshot writing rules

### `progress.md`

- 少なくとも次を分けて書く。
  - current capability snapshot
  - macro phase map
  - feature maturity matrix
  - layer / subsystem status
  - current self-driven line
  - user decision items vs research-discovery items
- 高い進捗率を書くときは、**current-L2 fixed-subset / non-production line に scoped した rough reading** だと明記する。
- `Mirrorea / Typed-Effect / Prism / 上位アプリ` を 1 行に潰さない。
- `shared-space docs-first boundary fixed` と
  `shared-space operational realization / final catalog open`
  を分けて書く。

### `tasks.md`

- 少なくとも次を分けて書く。
  - ordered self-driven packages
  - research-discovery items
  - user decision items
- 各 package では、可能なら
  - macro phase
  - feature family
  - current stage -> next stage
  - 完了条件
  - rough estimate
  を短く添える。
- `user が決める必要があること` と
  `研究で絞り込むべきこと`
  を混ぜない。

## Legacy checkpoint note

- 既存の `Phase 1 closeout fixed`、`Phase 6 front-half compile-ready checkpoint` などの表現は、
  historical / checkpoint anchor として引き続き使ってよい。
- ただし `progress.md` / `tasks.md` では、それだけを repo 全体の大局 phase 表現にしない。
- 両者が必要なときは
  - `legacy checkpoint`
  - `macro phase`
  のように明示的に書き分ける。
