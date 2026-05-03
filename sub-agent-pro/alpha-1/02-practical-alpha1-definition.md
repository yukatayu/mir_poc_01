# 02 — Practical Alpha-1 Definition

## 1. One-sentence definition

Mirrorea Spaces practical alpha-1 とは、Mir / Mirrorea の current theory に基づき、開発者が小さな仮想空間 product prototype を実際に作れる library-first toolchain である。

## 2. Alpha-1 is not final public product

Alpha-1 は final product ではない。以下はまだ final でなくてよい。

- final public grammar
- final public ABI
- production WAN federation
- durable distributed save/load
- full VRChat / VRM / Unity compatibility
- native binary execution completion
- Reversed Library
- production marketplace / package registry

ただし、alpha-1 は docs-only でも helper-only でもない。実用 prototype 開発ができる必要がある。

## 3. 100% 条件

Practical alpha-1 を 100% と呼ぶには、以下を全て満たす。

### 3.1 Source-to-runtime toolchain

- `.mir` または alpha-source を実際に front-door から読む。
- alpha grammar / parser は final でなくてよいが、documented であり sample に使われる。
- source -> IR -> checker -> runtime plan -> execution の path がある。
- sample-ID keyed bridge だけに依存しない。

### 3.2 Typed checking

最低限次を検査する。

- raw dangling reference reject
- guarded fallback access path
- inherited chain / snapshot selected / no implicit propagation
- read/write/mutable variance
- contract pre/post variance
- effect row containment
- failure row containment
- capability monotonicity
- observation label / redaction / retention constraints
- invalid cut predicate
- hot-plug package manifest admission

### 3.3 Runtime

- multiple Place を扱う reusable runtime API がある。
- local queue / MessageEnvelope / membership / capability / witness / event DAG を扱う。
- local world を起動し、object/layer/package を attach できる。
- sample world が library/CLI から実行できる。

### 3.4 Hot-plug

- HotPlugRequest / HotPlugVerdict / activation cut が actual runtime API で使える。
- debug layer / auth layer / rate-limit layer / object package の attach path がある。
- incompatible patch / missing capability / missing witness / stale membership が typed reject になる。
- detach minimal contract は少なくとも reject/defer/fallback の形で定義される。

### 3.5 Transport

- Docker/local TCP E2E が product-like command から実行できる。
- MessageEnvelope wire format は alpha schema として documented。
- auth / membership / capability / witness lanes は transport と分離される。
- production WAN は不要だが、local-only helper だけでは不十分。

### 3.6 Save/load

- local save/load is usable from CLI/library.
- local load は stale membership / stale lease / stale witness を hidden resurrect しない方針を明示。
- distributed save/load は未実装でもよいが、invalid distributed cut は checker/runtime-side で明示 reject される。

### 3.7 Devtools / visualization

- event DAG
- route trace
- membership timeline
- witness / publication / handoff relation
- hot-plug lifecycle
- fallback degradation
- redacted observer view

を JSON と viewer/HTML/devtools-style UI で確認できる。

### 3.8 Package / avatar compatibility skeleton

- avatar は core special case ではなく runtime package / adapter として扱う。
- placeholder + custom Mir avatar runtime が動く。
- unsupported runtime fallback がある。
- signed native package は safety proof ではないことを明記し、native execution は sandbox / policy later とする。

### 3.9 Product prototype

最低 1 つの small product prototype が動く。

推奨:

- `examples/practical-alpha1/worlds/sugoroku-space/`
- local + Docker mode
- debug layer attach
- rate-limit/auth layer attach
- avatar placeholder/custom package
- local save/load
- devtools export

## 4. 100% に含めないもの

- production WAN federation
- large-scale VRChat parity
- Reversed Library
- PrismCascade
- full native codegen
- final public packaging
- marketplace
- full theorem proving completion

## 5. Native output の判断

Alpha-1 で必要な native output は、Mir source が直接 machine code に compile されることではない。

Alpha-1 の native requirement は次である。

- Rust native runtime / CLI / library が build できる。
- Mir alpha source / IR bundle を native runtime が実行できる。
- optional runtime package manifest を native process が load/admit できる。

Direct native code generation is Alpha-2 or later unless user explicitly decides otherwise.

## 6. Success statement

Alpha-1 完了時に言えること:

> Mirrorea Spaces practical alpha-1 は、final public product ではないが、開発者が alpha source から小さな仮想空間 product prototype を作り、型検査し、local/Docker runtime で実行し、layer/package hot-plug し、devtools で観測し、local save/load できる toolchain である。
