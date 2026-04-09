# 126 — current L2 small decidable core and proof boundary inventory

## 目的

`specs/examples/30-current-l2-first-checker-cut-entry-criteria.md` と
`docs/reports/0358-async-control-memory-boundary-inventory.md` を前提に、

- current L2 / shared-space line で **language core の local / structural / decidable judgment**
  に残してよいもの
- **theorem prover / proof assistant** に送る方が自然なもの
- **model checker / protocol verifier** に送る方が自然なもの
- **runtime policy / operational layer** に残す方が自然なもの

を 1 本の inventory に集約する。

ここで固定するのは **Phase 5 current package の docs-first judgment** であり、

- final type system
- public checker API
- final theorem prover encoding
- final model checker input schema
- final async-control vocabulary

はまだ固定しない。

## scope

- current L2 の small decidable core を扱う。
- parser boundary / first checker cut / detached validation loop / shared-space baseline を前提にする。
- `atomic_cut` は place-local finalizing cut の最小核として扱い続ける。
- C++ 的 low-level memory-order family は current immediate candidate にしない。

## なぜ 4-way split が必要か

`static analysis / type / theorem prover / async-control boundary` を 2 分割だけで読むと、

- core checker を強くしすぎて syntax / runtime / proof を同時に固定しやすい
- 逆に core を薄くしすぎて current L2 の fail-closed structural floor が消えやすい
- `atomic_cut` の局所 cut と authority-serial transition / fairness / scheduler を同じ語彙で押し込みやすい

という問題が出る。

current package では、少なくとも次の 4 つを分ける方が自然である。

1. `core_static_checker`
2. `theorem_prover_boundary`
3. `protocol_verifier_boundary`
4. `runtime_policy_boundary`

## 比較した 3 案

### 案A — 強い static core を早く language 側へ入れる

#### 読み

- stronger capability / contract theory
- fairness / ordering に近い judgment
- global invariant proof の前提

までを early checker / early type system へ寄せる。

#### 利点

- user-facing には「言語だけでかなり守れる」ように見えやすい
- checker / type error の surface は一見一貫する

#### 欠点

- final parser grammar / public checker API / runtime gate を早く拘束しやすい
- current repo で docs-first に切っている staged line と衝突しやすい
- proof obligation と operational policy を language core へ押し込みやすい

### 案B — small decidable core + external verifier family を明示する

#### 読み

- local / structural / decidable 寄りの floor は `core_static_checker`
- canonical law や non-interference は `theorem_prover_boundary`
- membership / routing / authority / fairness protocol は `protocol_verifier_boundary`
- activation / auth / retention / retry などは `runtime_policy_boundary`

に分ける。

#### 利点

- current L2 の fail-closed floor を守りつつ、heavy workstream を早く既成事実化しない
- `atomic_cut` の局所性と higher-level async-control family を分けやすい
- parser boundary / detached validation loop / shared-space working subset と整合する

#### 欠点

- 4-way split の inventory が必要になる
- 「どこまで core か」を毎回明示する discipline が要る

### 案C — core は極薄、ほぼ全部を外部 verifier へ送る

#### 読み

- current docs で static rejection に寄せているものも language core から外し、
  representative helper / theorem prover / protocol tool に残す。

#### 利点

- language core は最も軽い
- future proof tooling との接続自由度は高い

#### 欠点

- current L2 の malformed / underdeclared fail-closed floor が薄くなる
- detached validation loop や static-only regression の前進量が小さくなる
- user-facing guidance が「何を言語が弾くか」を説明しにくい

## current first choice

current first choice は **案B** である。

つまり、

- **small decidable core は core checker に残す**
- **global invariant proof は theorem prover 側へ送る**
- **membership / routing / authority / fairness protocol は model checker / protocol verifier 側へ送る**
- **activation / auth / retry / retention / debug hook は runtime policy に残す**

という 4-way split を current package の inventory として採る。

## 4-way split の current inventory

### 1. `core_static_checker` に残すもの

次は current package で core checker 側に残してよい。

| cluster | representative anchor | current reading |
|---|---|---|
| same-lineage static evidence floor | fallback chain / `lineage(A -> B)` | local / structural / decidable floor |
| malformed / underdeclared rejection | missing edge / missing target / malformed clause attachment | hidden acceptance を防ぐ fail-closed floor |
| minimal capability strengthening prohibition | write 後段が前段より強い case | stronger capability theory までは入れない |
| request-local / option-local clause attachment | `require` / `ensure` / `admit` | role attachment の structural floor |
| minimal predicate fragment well-formedness | atom / application-like / `and` / grouping | richer grammar は外に残す |
| `try` / rollback locality structural floor | `try { ... } fallback { ... }`, `AtomicCut` node | dynamic gate ではなく structural floor に限る |

#### practical example A — current L2 fallback chain

```text
option writer on doc capability write lease live
  admit user_can_write

option readonly on doc capability read lease live

chain doc_ref = writer
  fallback readonly
    @ lineage(writer -> readonly)
```

ここで core checker に残すのは、

- `writer -> readonly` が same-lineage floor を満たすか
- capability strengthening が起きていないか
- `admit` が option-local marker になっているか

までである。

`user_can_write` そのものの意味や、canonical normalization の一般証明は core checker に押し込まない。

### 2. `theorem_prover_boundary` に送るもの

次は current package で theorem prover / proof assistant 側へ送る方が自然である。

| property | なぜ core checker に入れないか |
|---|---|
| canonical normalization law | local structural check だけでは足りない |
| no re-promotion | global semantic invariant である |
| rollback / cut non-interference | runtime relation 全体を要する |
| continuation と linear resource の一般安全性 | current phase では global proof obligation である |

#### practical example B — `try` / `atomic_cut`

```text
try {
  perform on doc_ref {
    ensure saved(doc_ref)
  }
  atomic_cut
  perform on audit_log {
    ensure notified(user)
  }
} fallback {
  perform on recovery_log
}
```

core checker に残すのは、

- `try` region shape
- `atomic_cut` が statement node として現れていること

までである。

一方で、

- `atomic_cut` 前後で rollback interference が起きないこと
- `saved(doc_ref)` と `notified(user)` を含む全 trace で no hidden rollback が保たれること

は theorem prover 側の property に残す。

### 3. `protocol_verifier_boundary` に送るもの

次は current package で model checker / protocol verifier 側へ送る方が自然である。

| property family | current reading |
|---|---|
| membership churn | join / leave / rejoin / reconnect protocol |
| authority handoff | room authority / provider binding の切替 protocol |
| routing / rebinding | Mirrorea / shared-space side の protocol property |
| fairness / witnessed draw protocol | auditable witness / provider receipt / replay obligation |
| distributed cut / durability | `atomic_cut` の外にある protocol property |

#### practical example C — authoritative room の witnessed draw

```text
control-plane:
  authority = room_authority_A
  rng_provider = provider_1

data-plane:
  request roll(player = P)
  draw_result = 4
  witness = {
    witness_kind = auditable_authority_witness,
    action_ref   = roll#42,
    draw_slot    = primary,
    draw_result  = 4
  }
```

ここで core checker が直接扱わないのは、

- authority handoff 後の stale witness reject
- provider receipt と witness の整合
- replay / fairness claim の protocol safety

である。これらは protocol / model checker 側に残す。

### 4. `runtime_policy_boundary` に残すもの

次は current package で runtime policy / operational layer に残す方が自然である。

| family | current reading |
|---|---|
| activation / reconciliation | actual admission / activation frontier |
| auth layering | principal identity と auth stack の接続 |
| retry / resend / timeout | room profile の外にある operational policy |
| artifact retention / bless | detached loop の path / retention policy |
| debug / graph export hook | replaceable observability layer |

#### practical example D — shared-space activation

```text
compile-time:
  role player may request roll
  role watcher may read positions

runtime control-plane:
  room_authority acknowledges join(user = U, role = player)
  activation frontier updates
  derived snapshot publishes active players
```

compile-time に残すのは role / capability / visibility の over-approximation だけであり、
actual activation frontier と retry / resend は runtime policy 側に残す。

## async-control family の current cut

current package では、

- `atomic_cut`
- authority-serial transition
- event-tree derived execution view
- witness-aware commit family

を同じ layer に押し込まない。

### `atomic_cut`

- current `place` の rollback frontier を確定する最小 cut primitive
- core checker には structural floor だけを残す

### authority-serial / witnessed / event-tree family

- room-level ordering
- fairness witness
- replay obligation
- derived execution / debug / explanation view

として扱い、current core checker へは入れない。

### low-level memory-order について

current package では、C++ の `std::memory_order` 的な low-level vocabulary を immediate candidate にしない。

理由は、

- current repo の source-backed semantics がそこまで進んでいない
- parser / checker / runtime / proof burden を同時に重くしやすい
- current shared-space line は authority / consistency / witness-aware commit family の方が自然だから

である。

## current package でまだ固定しないもの

- final type system strength
- public checker artifact / reason code final shape
- theorem prover 向け core relation の最終 encoding
- model checker input schema の最終形
- final async-control vocabulary
- low-level memory-order family を将来 external verifier vocabulary としてだけ残すかどうか

## current judgment

- **current first choice は `small decidable core + external verifier family explicit split` である**
- **core checker には local / structural / decidable floor だけを残す**
- **global semantic invariant は theorem prover 側へ送る**
- **membership / routing / authority / fairness protocol は model checker / protocol verifier 側へ送る**
- **activation / auth / retry / retention / debug hook は runtime policy に残す**
- **`atomic_cut` は current core の最小核に留め、higher-level async-control family は別 line として扱う**

したがって current package では、
Phase 5 を「強い型システムの即時 actualization」ではなく、
**small decidable core を守ったまま proof obligation と protocol obligation の境界を inventory 化する段階**
として読むのが最も自然である。
