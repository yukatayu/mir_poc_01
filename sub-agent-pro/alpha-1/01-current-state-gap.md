# 01 — Current State and Gap Analysis

## 1. 現状の正しい評価

現行 repo は、docs-only 構想ではない。次の evidence がある。

- current-L2 / clean-near-end runnable floor
- Sugoroku vertical slice
- avatar follow representative slice
- typed external preview
- network canary
- projection/codegen manifest bridge
- viewer prototype inventory
- hot-plug narrow runtime floor
- alpha local runtime narrow floor
- alpha layer insertion floor
- alpha Docker/local transport floor
- alpha avatar/package admission floor
- alpha local-only save/load floor
- alpha visualization subset
- alpha thin integrated E2E bridge

しかし、それらは主に次の性質を持つ。

- helper-local
- report-local
- runtime-private
- non-public
- narrow
- sample-ID keyed
- sidecar-backed
- thin bridge
- current-scope closeout

したがって、これは実用製品基盤そのものではない。

## 2. 現行 `100%` の意味

現行 `progress.md` の Stage A..F `100%` は、current-scope closeout の意味である。これは以下を意味しない。

- final public parser grammar
- `.mir` source front-door 実行
- public checker / runtime / verifier API
- installed binary / packaging target
- native output / native execution completion
- production network transport
- durable distributed save/load
- final viewer / telemetry API
- final hot-plug ABI
- product-ready Mirrorea Client
- VRChat-class feature completion
- Reversed Library implementation

今後は `100%` を実用可能 alpha-1 に合わせて再定義する。

## 3. 実用可能 100% との乖離

実用可能 alpha-1 には少なくとも次が必要である。

- source front-door: sample source から parse / typed IR / check / run できる
- checker: lifetime/fallback, contract variance, effect/failure row, capability, cut predicate を一通り検査できる
- runtime: sample-ID keyed bridge ではない reusable local runtime API
- package: world/object/layer/runtime package manifest と admission checker
- hot-plug: request / verdict / activation cut / detach-minimal contract
- transport: Docker/local TCP 以上の reproducible E2E path
- devtools: event DAG / route / witness / membership / hot-plug trace export と viewer
- save/load: local save/load + distributed invalid cut rejection + clear later boundary
- CLI/library: product prototype 開発に使える command / library surface
- docs: practical alpha-1 の使い方、sample、limitations

現状は多くの evidence があるが、これらが一体化した実用 toolchain ではない。

## 4. 誤解を避けるための新しい metric

`progress.md` と `samples_progress.md` に以下を導入する。

- `evidence closeout`: 現在の current-scope closeout。既存 Stage A..F をここに移す。
- `practical alpha-1 readiness`: 実用可能 alpha-1 に対する進捗。
- `public product readiness`: final public product / U1 後の進捗。今は別 gate。

Stage A..F の既存 100% は `evidence closeout` として保持し、practical readiness の 100% とは別にする。

## 5. 現時点の practical alpha-1 readiness の暫定評価

初期値は慎重に置く。

- Mir theory / sample evidence: 70-80%
- source front-door / parser: 10-20%
- typed IR / checker reusable toolchain: 20-35%
- reusable runtime API: 25-40%
- transport practical E2E: 35-45%
- package/hot-plug practical API: 25-40%
- devtools practical viewer: 30-45%
- save/load practical local: 30-45%
- product scaffold / client: 5-15%
- overall practical alpha-1: 25-35%

Codex は exact numbers を repo evidence に合わせて調整してよいが、current-scope 100% をそのまま practical 100% にしてはいけない。
