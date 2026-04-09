# Phase 1 要約 — current L2 semantics stabilization

## 何をした phase か

Phase 1 は、current L2 の semantics core を
**parser より先に安定化する phase** である。

ここで主に固めたのは、次の読みである。

- fallback は guarded option chain
- degradation は left-to-right monotone
- no re-promotion
- request-local clause と option-local clause を分ける
- `TryFallback` / `AtomicCut` は structural floor と runtime / proof boundary を分ける

ここでいう option chain の `fallback` は候補降格であり、`try { ... } fallback { ... }` の回復 branch とは別 layer の fallback である。

## 中心にある考え方

### fallback

```text
chain ref = writer
  fallback delegated_writer
  fallback readonly
```

これは「外側 wrapper が入れ子になる」のではなく、
**同じ chain の候補列を左から右へ降格する**と読む。

したがって、途中で一度弱い候補へ落ちた後に強い候補へ戻ることはない。

### request-local / option-local

```text
perform write on doc
  require user_can_write
  ensure write_committed

option writer on doc capability write lease live
  admit user_can_write
```

- `require` / `ensure` は request-local
- `admit` は option-local

であり、同じ predicate を見ていても役割が違う。
また、`perform` は request 実行、`option` は候補宣言であり、`writer` や `write` は representative example の識別子であって built-in 関数名ではない。

### try / atomic_cut

```text
try {
  perform write via writer_chain
  atomic_cut
  perform notify on room
} fallback {
  perform enqueue on retry_queue
}
```

ここで Phase 1 が先に固めたのは、

- `try { ... } fallback { ... }` という structural role
- `atomic_cut` が rollback frontier に関与する node であること

までである。

一方で、

- restore scope の一般証明
- distributed protocol を含む rollback safety

はまだ外へ残している。
`require` は request 前提、`ensure` は commit 条件、`admit` は option の入場条件であり、`atomic_cut` は `try` 専用構文ではないが active rollback frame があるときに強く効く。

## この phase で得たもの

- representative example を安定して読める semantics core
- prose drift を抑える 기준
- parser-free PoC が依存できる意味論土台

## この phase で意図的に決めていないもの

- final parser grammar
- 強い型システム
- theorem prover / model checker の final boundary

つまり、Phase 1 は「意味を先に固定する」phase であり、
「全部の surface や proof 方法を決める」phase ではない。

## 次 phase へ渡したもの

Phase 2 は、この semantics core を parser-free fixture / harness / detached loop で回す。
Phase 3 は、この semantics core を壊さずに parser boundary と first checker cut を narrow に切る。
