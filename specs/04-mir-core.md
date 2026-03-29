# 04 — Mir Core

## Status

Mir Core is the most foundational part of the project.
Its architecture should be treated as decision level L0/L1 unless explicitly marked otherwise.

This document distinguishes between:

- **Mir as a whole**: the broader semantic foundation that also constrains later evolution and integration work.
- **Mir-0**: the minimum semantic kernel that should be stabilized first.

Unless explicitly marked otherwise, the "Mir-0 minimum semantic core" section below is the current focal definition of Mir-0 for Workstream A.
It does not replace the earlier normative documents; it is read together with `specs/01` through `specs/03` and `specs/09`.

## The four pillars

### 1. Causality and Time
- Computation is represented as a directed acyclic graph of events.
- Finalization boundaries matter semantically.
- The broader Mir cut vocabulary previously mentioned `barrier`, `atomic_cut`, and `durable_cut`. Mir-0 fixes only `atomic_cut`; the status of `barrier` and `durable_cut` is deferred.
- The system must be able to explain "what happened before what" and "what became final when".

### 2. Effects and Contracts
- External actions are first-class effects, not hidden implementation details.
- Contracts (`require`, `ensure`, `invariant`) are part of executable meaning.
- Failure is not a generic exception bucket; it is structured and contract-aware.

### 3. Ownership and Lifetime
- Linear or ownership-like resources matter.
- Lifetimes and fallback-style degradation must remain monotone.
- Preference chains and guarded references are currently an active design topic.

### 4. Safe Evolution
- Patches should preserve meaning and respect graph discipline.
- Default rule: evolve by downstream addition rather than arbitrary rewiring.
- Overlays must preserve compatibility and must not silently shadow existing APIs.

Mir-0 does not attempt to fix the complete operational semantics of patches, overlays, or route rebinding.
It fixes only the semantic constraints that later mechanisms must obey.

## Mir-0 minimum semantic core

The minimum kernel fixed here is intentionally narrow.
It includes only the primitives needed to state causality, explicit effects, rollback constraints, and ownership discipline without importing Mirrorea- or Prism-specific machinery.

### 1. Event DAG and causality

- Mir-0 computation is represented as a directed acyclic graph of semantic events.
- The graph must make causal explanation explicit: which event happened before which other event, and which boundary made an earlier prefix final.
- Hidden backward edges in meaning are not permitted.

### 2. `place`

- Mir-0 includes a minimum notion of `place`.
- A `place` is the smallest semantic locus at which local state, rollback scope, and ownership transfer are interpreted.
- Mir-0 requires enough structure to distinguish same-place reasoning from cross-place interaction.
- Exact routing, naming, and distributed placement policy are outside Mir-0 and belong to later Mir or Mirrorea work.

### 3. `perform`, effect, and contract

- Mir-0 では、明示された contract の下で effect を要求する最小操作だけを仮定する。
- 本文ではその操作を便宜上 `perform` と表記するが、これは Mir-0 の規範的な表面構文としてはまだ確定しない。
- `perform` を最終的な予約語として採用するかどうかは **未決定** である。
- Effects are first-class semantic actions, not hidden implementation details.
- Contracts constrain both admission and outcome space. Current vocabulary includes `require`, `ensure`, and `invariant`, but the exact surface syntax is not fixed here.
- Effect success, rejection, approximation, and compensation must remain visible in the event graph.

### 4. Minimum failure space

- Mir-0 fixes a minimum structured failure space with at least these named outcomes:
  - `Reject`
  - `Approximate`
  - `Compensate`
- A contract must state which degraded outcomes are permitted.
- The exact lattice-theoretic presentation beyond these minimum classes is **UNRESOLVED**.

### 5. Primitive fallback

- Mir-0 includes fallback as a primitive way to move to the next contract-compatible option after explicit failure or monotone degradation.
- Fallback must not introduce hidden backtracking, hidden API shadowing, or duplication of linear resources.
- Full normalization of nested fallback / preference chains is outside Mir-0.

### 6. `try` with local rollback

- Mir-0 includes `try` with local rollback semantics.
- Rollback is local to the current `place` and may only revert state that has not crossed a finalizing cut.
- Mir-0 における `try` の rollback frontier は current `place` 内で閉じており、途中に `atomic_cut` があればそこで停止する。
- Non-rollbackable effects must be prohibited inside the rollback region, isolated from it, or handled by explicit compensation.
- Rollback is not a hidden control-flow trick; it is part of the explicit failure/effect semantics.

### 7. `atomic_cut`

- Mir-0 includes `atomic_cut` as the minimum place-local finalizing cut primitive.
- Mir-0 において `atomic_cut` が確定するのは current `place` の rollback frontier だけである。
- `atomic_cut` は単一プロセス全体の同期点、複数 `place` をまたぐ合意点、永続化完了点を意味しない。
- Within a `place`, an `atomic_cut` creates an explicit decision boundary after which rollback may not pass.
- `atomic_cut` の後で failure が表面化しても、同じ `place` の pre-cut 部分は rollback されない。以後は compensation、fallback、または明示的な failure として扱う。
- Mir-0 fixes the rollback/finalization role of `atomic_cut`, but it does not use `atomic_cut` alone to decide persistence or distributed finalization semantics.
- If later computation needs to account for pre-cut effects, it must do so through explicit continuation or compensation rather than implicit rollback across the cut.

### 8. Linear resources and monotone lifetime

- Mir-0 includes the minimum linear-resource discipline needed to prevent duplication by rollback, fallback, or later evolution machinery.
- Ownership transfer must remain explicit.
- Lifetime degradation is monotone.
- Any later convenience layer must preserve these rules rather than weaken them.

## Topics intentionally outside Mir-0

- `durable_cut` は Mir-0 に含めない。Mir-1 側では、`atomic_cut` に abstract persistence requirement を伴う durable commit guarantee を追加する cut vocabulary 候補として扱う。
- `durable_cut` の最小意味は、successful として観測された pre-cut prefix が local rollback、process restart、route rebinding の後に未確定状態へ戻らないことである。Mir-1 はこの guarantee を意味づけるが、具体的な storage / replication / consensus mechanism は固定しない。
- `durable_cut` の failure は、durable guarantee を確立できず、その cut attempt を成功した cut として確定できなかったことを意味する。この段階では既存の failure lattice を再利用し、新しい failure class は導入しない。
- 既定では、durable guarantee を確立できなかった `durable_cut` は `Reject` として扱う。cut 後の外部化された obligations を明示的に巻き戻す必要がある場合だけ `Compensate` を使う。`Approximate` は contract が durability を弱めた代替を明示的に許す場合を除き、`durable_cut` failure の既定分類にはしない。
- local durable failure と distributed durable failure は、Mir-1 では別 failure class に分けない。どちらも同じ durable guarantee failure の実現上の差分であり、後者の distributed finalization は Mirrorea 側の実現責務に属する。
- distributed finalization は `durable_cut` の最小意味そのものではない。複数 `place` や実ノードをまたぐ `durable_cut` を成立させるときの operational realization 側に属する。
- Mirrorea は `durable_cut` を意味づける場所ではない。Mir-1 で定められた durable commit guarantee を、storage / replication / distributed cut realization / audit 上で実現する責務を持つ。
- `barrier` は Mir-0 の cut vocabulary に含めない。Mir-1 に残すとすれば、commit-like primitive ではなく ordering primitive 候補として扱う。
- **未決定**: `barrier` が Mir-1 で独立語彙として本当に必要か、それとも他の ordering / cut 構成に吸収されるか。
- **未決定**: `durable_cut` の persistence evidence をどの形式で観測・検証するか。
- **未決定**: contract が durability を弱めた代替結果を本当に許す場合、その degraded path を `Approximate` としてどこまで許容するか。
- **UNRESOLVED**: full fallback normalization and the complete algebra of preference chains.
- **UNRESOLVED**: exact relationship between `emit`, effect handlers, and structured event routing.
- **UNRESOLVED**: coroutine support, including suspension restrictions and interaction with cuts, rollback, and patching.
- **UNRESOLVED**: overlay alias details, route rebinding details, and other Mirrorea-dependent safe-evolution mechanisms.

These are still part of broader Mir design space, but they are not part of the minimum kernel being fixed in this pass.

## What Mir is not
- It is not the same thing as the deployment fabric.
- It is not the same thing as the route / overlay control plane.
- It is not a domain-specific media runtime.
- It is not a virtual-reality engine.
- It is not only a type system; it also includes operational boundaries and evolution rules.
