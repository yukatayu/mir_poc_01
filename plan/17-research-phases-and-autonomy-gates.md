# plan/17 — research phases and autonomy gates

## 目的

この文書は、repo 全体の長期研究を

- macro phase
- autonomy gate
- stop condition

で見やすく整理する。

ここでいう phase は strict waterfall ではない。
複数 line の並走を前提にする。

## legacy checkpoint の扱い

- `specs/examples/...` や report に出てくる `Phase 1..7` は historical label として残す。
- repo 全体の current status には、それとは別に macro phase を使う。

## macro phase 一覧

| Macro phase | 主眼 | 現在位置 | 重さ | autonomy gate | typical stop condition |
|---|---|---|---|---|---|
| `Macro 0` | repository memory / docs / traceability | maintenance | 低 | self-driven | snapshot/detail-side drift が増えたら更新 |
| `Macro 1` | semantic kernel / invariant stabilization | late | 中 | self-driven | L0/L1 invariant へ影響する変更が出る |
| `Macro 2` | parser-free validation substrate | late | 中 | self-driven | helper/public drift が出る |
| `Macro 3` | compile-ready minimal actualization | late | 中 | self-driven | support-only/public-candidate split が崩れる |
| `Macro 4` | executable fixed-subset sample expansion | active | 重 | self-driven | new sample family が core semantics を押し広げる |
| `Macro 5` | typed / theorem / model-check bridge | active at boundary | 重 | self-driven up to boundary | concrete tool / full type calculus / production contract が必要になる |
| `Macro 6` | fabric / shared-space / runtime evolution | docs-first boundary only | 重 | mixed | final operational catalog / policy profile が必要になる |
| `Macro 7` | toolchain / backend / host-facing integration | thin facade plus reserve shell | 重 | mixed | final public packaging / external target が必要になる |
| `Macro 8` | domain / application realization | not started | とても重い | user spec required | first application target が必要になる |

## autonomy gate detail

### self-driven でよい line

- semantic kernel の narrow refinement
- parser-free regression / helper maintenance
- compile-ready minimal actualization の narrow widening
- fixed-subset source sample expansion
- typed attachment candidate inventory
- theorem pilot planning
- sample-visible theorem/model-check boundary hardening

### boundary までは self-driven でよい line

- model-check projection / property-family reserve planning
- shared-space docs-first boundary
- docs-first I/O / host-facing boundary
- ordering / `memory_order` reinterpretation の theory-first inventory

### mixed gate の line

- concrete theorem prover / model-check tool binding
- shared-space room policy / fairness / replay operational profile
- host binding artifact から concrete adapter target への移行
- public operational shell から final packaging への移行

### user specification が必要な line

- shared-space final activation / authority / auth / consistency / fairness catalog
- first external integration target
- backend / tooling success criteria
- upper-layer application target

## current lines

### current promoted line

1. `Macro 4` stable malformed capability second source-backed widening actualization

### current reserve line

- `Macro 7` public operational CLI concrete shell actualization

### active adjacent research line

- `Macro 5` typed / theorem / model-check detailed planning
- `Macro 6/7` shared-space / host-I/O reserve hardening
- `Macro 5/6` ordering boundary inventory

## current stop lines

次はまだ finalization しない。

- final parser grammar
- final public parser / checker / runtime API
- full strong type system
- concrete theorem / model-check production contract
- shared-space final catalog
- backend / codegen public surface
- raw FFI / game engine direct binding actualization
- upper-layer application contract

## current judgments

- current repo は architecture-first だが、fixed-subset runnable path を already 持つ。
- type/proof/model-check line は「まだ無理」ではなく、boundary/pilot planning を進められる段階にある。
- ordering / `memory_order` reinterpretation も「未着手で何も言えない」段階ではなく、theory-first inventory と handoff boundary を整理できる段階にある。
- ただし、この 2 系統はいずれも mainline implementation へ即昇格させる段階ではない。
