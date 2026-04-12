# plan/10 — 全体ロードマップ

## 目的

この文書は、repo 全体の研究を

- 何を先に固めるか
- 何を後段へ残すか
- old `FutureWork` bucket をどう分解するか

の観点で整理する。

## current overall reading

現在の repo は、次の 3 つを同時に持っている。

1. semantics / invariants / boundary の docs-first repo
2. parser-free current L2 の runnable validation substrate
3. compile-ready minimal actualization と fixed-subset source sample の narrow runnable path

したがって、現在地を
「まだ implementation 前」
とも
「すでに broad implementation 期」
とも読むのは不正確である。

自然な読みは、
**architecture-first だが、fixed subset は理論と実行を ratchet で並走させている**
である。

## macro phase roadmap

### `Macro 0` repository memory

- docs / plan / report / snapshot / FAQ / AGENTS / `.docs`
- これは常時 maintenance phase である

### `Macro 1` semantic kernel

- Mir core semantics
- invariants
- fallback / `lease` / `atomic_cut`
- current L2 notation boundary

### `Macro 2` parser-free validation substrate

- fixture corpus
- interpreter
- host harness
- detached validation loop
- regression baseline

### `Macro 3` compile-ready minimal actualization

- `mir-ast`
- `mir-semantics`
- `mir-runtime`
- tool-neutral formal hook

### `Macro 4` executable fixed-subset sample expansion

- source corpus
- mapping / lowering / runner
- verification ladder
- authored row widening

### `Macro 5` static reasoning / theorem / model-check bridge

- proof obligation boundary
- proof notebook bridge
- concrete theorem / model-check pilot
- later type / static-analysis / external verifier line

### `Macro 6` fabric / shared-space / runtime evolution

- Mirrorea
- shared-space
- dynamic attach / detach
- authority / consistency / fairness boundary

### `Macro 7` toolchain / backend / public dev surface

- public operational CLI
- backend / codegen
- editor / LSP / graph tooling

### `Macro 8` domain / application realization

- synchronized shared-space example
- collaborative editing
- virtual-world / avatar-oriented line
- other concrete application targets

## なぜ old `Phase 7` を分割するのか

old `Phase 7` には、

- fabric
- shared-space
- Typed-Effect
- Prism
- backend
- tooling
- applications

が混在していた。

しかしこれらは、

- self-driven に boundary まで詰められるか
- user specification が必要か
- implementation cost
- current code / sample line との近さ

が大きく異なる。

そのため current roadmap では、

- `Macro 6` fabric / shared-space
- `Macro 7` toolchain / backend
- `Macro 8` domain / application

へ分ける。

## near-term mainline

近接 mainline は次である。

1. Mirrorea / shared-space docs-first re-entry
2. model-check / public-checker second reserve inventory
3. stable-static edge-pair first reopen
4. public operational surface actualization gate

これは主に `Macro 4` と `Macro 5` の line である。

## まだ急がないもの

- final parser grammar
- final public parser / checker / runtime API
- LLVM-family backend / external codegen
- shared-space final catalog
- upper-layer application finalization

## current recommendation

- fixed-subset runnable path を厚くすること自体は今やってよい。
- theorem / model-check bridge も narrow pilot までは今やってよい。
- ただし higher-level async-control / memory-order family、fabric operational realization、backend public surface は separate track として扱う。
