# plan/10 — 全体ロードマップ

## 目的

この文書は、repo 全体の研究を

- どこまで current executable mainline として進めるか
- どこから先を adjacent research track に分けるか
- old `FutureWork` bucket をどう解体するか

の観点で整理する。

## current overall reading

現在の repo は、次の 3 つを同時に持つ。

1. semantics / invariants / boundary の docs-first repo
2. parser-free current L2 の runnable validation substrate
3. compile-ready minimal actualization と fixed-subset source sample の narrow runnable path

したがって current reading は、
**architecture-first だが、fixed subset は理論と実行を ratchet で並走させている**
である。

## macro roadmap

| Macro phase | 主眼 | 現在位置 | 次の exit signal |
|---|---|---|---|
| `Macro 0` | repository memory / docs / traceability | maintenance | snapshot と detail-side plan の drift suppression |
| `Macro 1` | semantic kernel / invariant stabilization | late | current L2 semantic floor の追加 reopen が narrow になる |
| `Macro 2` | parser-free validation substrate | late | detached loop / fixture corpus / compare helper の drift suppression |
| `Macro 3` | compile-ready minimal actualization | late | support-only/public-candidate split 安定化 |
| `Macro 4` | executable fixed-subset sample expansion | active | next malformed / widened rows を current ladder に安全に追加できる |
| `Macro 5` | typed / theorem / model-check bridge | active at boundary | typed attachment / theorem pilot / model-check projection の first planning cut 固定 |
| `Macro 6` | fabric / shared-space / runtime evolution | docs-first boundary only | room-profile / confusion-replay / authority-family の compact boundary note 固定 |
| `Macro 7` | toolchain / backend / host-facing integration | thin facade plus reserve shell | shell actual package と bridge-only host note の boundary 固定 |
| `Macro 8` | domain / application realization | not started | first target profile が user と合意される |

## old `Phase 7` を分割した current reading

old `Phase 7` は次を 1 つに押し込んでいた。

- fabric / shared-space
- toolchain / backend
- host-facing integration
- application / demo target

現在はこれを、

- `Macro 6` fabric / shared-space
- `Macro 7` toolchain / backend / host-facing integration
- `Macro 8` domain / application realization

へ分ける。

## current executable mainline

1. `Macro 4` stable malformed capability second source-backed widening actualization
2. `Macro 7` reserve public operational CLI concrete shell actualization

## adjacent research tracks

### Track A. typed / theorem / model-check

- いま進めてよいのは、
  - obligation allocation
  - typed attachment candidate inventory
  - semantic-core theorem pilot plan
  - model-check projection / property-family plan
  までである。
- これは **見通しが厳しすぎて止まっている線ではない**。
  current repo には formal hook、review-unit pilot、machine-facing carrier、sample-facing summaryが already あるので、boundary/pilot planning は進められる。
- concrete tool binding や full type calculus は still later である。

### Track B. shared-space / host-facing integration

- shared-space は identity / admission / authority までは docs-first boundary がある。
- host-facing I/O は privileged `stdin/stdout` ではなく capability-scoped port / adapter boundary の docs-first cut がある。
- ここから先は room-profile、confusion/replay、binding artifact を narrow に足せる。
- final operational catalog、concrete adapter target、application profile は later / mixed gate である。

### Track C. ordering / `memory_order` / syntax / modality theory line

- `atomic_cut` の place-local nucleus は current executable core にある。
- その先の higher-level ordering / fairness / witness-aware handoff / authority-serial family は **theory-first** であり、detail-side plan を進める段階にある。
- syntax honesty と modal foundation も、この line と同じく **comparison / adequacy / stop-line planning** を先に進める段階にある。
- したがって、ここも **研究不能ではない** が、implementation-ready ではない。

## まだ急がないもの

- final parser grammar
- final public parser / checker / runtime API
- concrete theorem prover / model-check tool binding
- LLVM-family backend / external codegen
- shared-space final catalog
- raw FFI / game engine direct binding actualization
- upper-layer application finalization

## autonomous research rhythm

current repo では、次の ratchet を基本にする。

1. docs-first comparison / threshold を切る
2. narrow pilot または helper-local actual evidence を足す
3. regression / smoke / detached loop を回す
4. `plan/` / `progress.md` / `tasks.md` / relevant docs を同期する

この rhythm を壊す broad actualization は避ける。
