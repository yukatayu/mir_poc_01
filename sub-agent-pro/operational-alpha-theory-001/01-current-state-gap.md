# 01 — current state and gap

## 現状の大局

repo は docs-only 構想メモではない。Mir current-L2、clean-near-end、Sugoroku、avatar follow、typed external preview、network canary、projection/codegen bridge、viewer inventory、hot-plug narrow floor、Alpha-0 / practical alpha-1 first floors は、多くが runner / helper / Rust floor / expected report によって確認できる。

一方で、ユーザが期待する「実用できる α-0.5 / α-0.8」はまだ満たされていない。現状は、exact report や preview bundle を組み合わせた first floors が多いが、開発者が同一 runtime session 上で check / run / observe / save / hot-plug を行う operational toolchain にはまだ gap がある。

## current evidence categories

### active clean suite

- `samples/clean-near-end/`
- current canonical executable suite
- typing / order-handoff / model-check / modal / Sugoroku / avatar follow など

### alpha-0 evidence root

- `samples/alpha/`
- active runnable root ではない
- checker-seed / helper-local acceptance / runtime-mirror / runner-backed non-public floor が混在

### practical alpha-1 first floors

- `samples/practical-alpha1/`
- limited `package.mir.json` front-door
- checker floor
- local runtime floor
- hot-plug floor
- transport floor
- save/load floor
- devtools export floor
- avatar preview floor
- product preview bundle floor

## primary gap

現状にあるもの:

```text
package.mir.json -> limited loader
selected checker floor
selected runtime report
selected hot-plug report
selected transport report
selected save/load report
selected devtools export bundle
selected product preview bundle
```

まだ弱いもの:

```text
same-session operational runtime
host input -> typed external adapter -> runtime -> output route
a single developer workflow: check -> run -> observe -> save/load -> attach -> observe
session-bound devtools
operational α-0.5 / α-0.8 completion criteria
```

## FAQ 015 による重要整理

FAQ 015 は次を整理している。

- generic host input ingestion / transform / host output emission はまだ mainline にない
- `samples/clean-near-end/sugoroku-world/` は runnable だが single OS process logical multi-place emulator
- practical alpha-1 transport は local TCP / Docker Compose TCP first floor
- hot-plug request/verdict/activation cut の first floor はあるが network-wide patch distribution や simultaneous activation はない
- debug / visualization は optional polish ではなく project axis に入る必須層
- package slicing は意図的だが artifact-based milestone が前に出すぎた
- `alpha-1 complete` のような裸の表現は避け、category を分けるべき

## correction to planning

今後は次を分ける。

```text
evidence closeout
first-floor closeout
operational-layer-ready
public/product readiness
```

α-0.5 / α-0.8 は `operational-layer-ready` で判定する。

