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
- sample-visible theorem/model-check evidence
- later type / static-analysis / external verifier line

### `Macro 6` fabric / shared-space / runtime evolution

- Mirrorea
- shared-space
- dynamic attach / detach
- authority / consistency / fairness boundary

### `Macro 7` toolchain / backend / public dev surface

- public operational CLI
- host-facing I/O / adapter boundary
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
- `Macro 7` toolchain / backend / host-facing integration
- `Macro 8` domain / application

へ分ける。

## near-term mainline

近接 mainline は次である。

1. public operational CLI concrete shell actualization comparison
2. reserve stable malformed capability second source-backed widening actualization

ここでの自然な読みは、**shared-space docs-first follow-up の checkpoint を `specs/examples/375...376` までで一旦閉じ、その後 `specs/examples/377...384` で fixed / actualize した carrier line に従って `Macro 5` の sample-visible theorem/model-check milestone を閉じ、`specs/examples/385...386` で `Macro 7` の docs-first host-facing integration boundary を固定し、`specs/examples/387...388` で `Macro 4` の missing-option actualization comparison を source-backed widening first に寄せ、`specs/examples/389...390` で final public parser/checker/runtime first later gate を runtime-led thin facade に絞り、`specs/examples/391...392` で `e16/e18` source-backed widening first cut を閉じ、`specs/examples/393...394` で public operational CLI second gate を Rust-side operational wrapper over thin facade として narrow に固定し、`specs/examples/395...396` で thin-facade later support cut を `run_current_l2_runtime_skeleton` に置き、`specs/examples/397...398` で capability second reopen comparison を `e13/e20` pair judgment + source-backed widening first に固定し、`specs/examples/399...400` で current-L2 scoped docs-only shell naming を `mir-current-l2 run-source-sample <sample> --host-plan <path> --format pretty|json` に narrow に固定したうえで、次段を capability source-backed widening actualization comparison に送る** である。

## near-term reserve line

- stable malformed capability second source-backed widening actualization

## まだ急がないもの

- final parser grammar
- final public parser / checker / runtime API
- LLVM-family backend / external codegen
- shared-space final catalog
- upper-layer application finalization
- duplicate cluster と `TryFallback` / `AtomicCut` malformed-static family の broader source-backed promotion

## current recommendation

- fixed-subset runnable path を厚くすること自体は今やってよい。
- theorem / model-check bridge も sample-visible milestone までは今やってよい。
- host-facing I/O / adapter boundary は、`specs/examples/385...386` で privileged `stdin/stdout` を避けた capability-scoped port / adapter boundary として docs-first に切る方針を fixed 済みである。
- FFI や game engine integration は否定しないが、direct binding を先に入れず、visualizer / host substrate / FFI / engine adapter を separate gate に置く。
- higher-level async-control / memory-order family、fabric operational realization、backend public surface は separate track として扱う。
