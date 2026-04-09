# current L2 読み方メモ

この文書は、Phase 1〜3 の要約を読むときに起きやすい短い疑問を先回りして解くための補助メモである。
規範判断の正本ではなく、**research_abstract を読みやすくするための companion** として置く。

## 1. `fallback` が 2 種類あるのはなぜか

current L2 の要約に出てくる `fallback` には、少なくとも次の 2 つがある。

```text
chain doc_ref = writer
  fallback readonly

try {
  perform write via doc_ref
} fallback {
  perform enqueue on retry_queue
}
```

- 1 つ目は **option chain の fallback**
  - same-lineage の候補列を左から右へ降格していく
  - 途中で earlier option へ戻らない
- 2 つ目は **`try` の fallback**
  - body failure を受けて local rollback 後に別 branch を実行する

両者は「失敗時に後段を試す」という広い意味では類似しているが、**同じものではない**。

- option chain fallback
  - 候補選択 / admissibility / degradation の話
- `try` fallback
  - control flow / rollback / recovery branch の話

したがって、名前がたまたま被っただけではなく、**近い回復系の語彙だが、理論上は別 layer の fallback** と読むのが current repo の整理である。

## 2. `writer` や `write` は言語組み込みか

基本的に、research_abstract に出てくる

- `writer`
- `delegated_writer`
- `readonly`
- `write`
- `notify`
- `retry_queue`

などは、**representative example の中で使っている識別子**であり、言語組み込み関数ではない。

組み込みに近い current L2 companion 語彙は、たとえば次である。

- `perform`
- `option`
- `try`
- `fallback`
- `require`
- `ensure`
- `admit`
- `atomic_cut`

ただし、これも **final parser grammar を固定したという意味ではない**。current phase では companion notation / representative notation として使っている。

## 3. `perform` と `option` はどう違うか

短く言うと、

- `option` は **候補の宣言**
- `perform` は **実際の request 実行**

である。

例:

```text
option writer on doc capability write lease live
  admit user_can_write

perform write via doc_ref
```

- `option writer ...`
  - 「`writer` という候補は、`doc` に対する `write` capability を持ち、`lease` と `admit` 条件を持つ」と宣言している
- `perform write via doc_ref`
  - 実際に request を投げ、chain を辿って admissible な候補で実行する

## 4. `require` / `ensure` / `admit` は何か

### `require`

`require` は **request-local precondition** である。

```text
perform write via doc_ref
  require user_can_write
```

この request を続けてよいかを判定する。

### `ensure`

`ensure` は **request-local postcondition / commit condition** である。

```text
perform write via doc_ref
  ensure write_committed
```

effect attempt が success-side carrier を返した後で、それを本当に commit してよいかを見る。
current L2 では、**`ensure` が通る前に可視 state を commit しない**。

### `admit`

`admit` は **option-local admissibility gate** である。

```text
option writer on doc capability write lease live
  admit user_can_write
```

これは「この option 自体が候補として選べるか」を見る。
`admit` miss は request 全体の即 failure ではなく、current L2 では **その option を skip して後段候補へ進む理由**になる。

要するに、

- `admit`
  - option が候補として入場できるか
- `require`
  - request をその候補で進めてよいか
- `ensure`
  - success-side carrier を commit してよいか

で役割が違う。

## 5. `atomic_cut` には `try` が必須か

**必須ではない。**

```text
perform update_authority on authority_cell
atomic_cut
perform append_audit on audit_log
```

このように `try` の外で現れてよい。

ただし current L2 で `atomic_cut` が強く効くのは、**active rollback frame があるとき**である。

- `try` の中で matching rollback frame がある
  - rollback frontier を更新する
- active rollback frame が無い
  - event / audit 上の marker に留まりうる

### 影響範囲

`atomic_cut` の影響範囲は **place-local / current rollback region local** であり、
「同期範囲が推移的に全体へ波及する cut」ではない。

したがって current L2 では、

- global barrier
- distributed consensus cut
- transitive global freeze

のようには読まない。

## 6. `lineage` と、その前の `@` は何か

例:

```text
chain doc_ref = writer
  fallback readonly
    @ lineage(writer -> readonly)
```

- `lineage(writer -> readonly)`
  - **edge-local metadata**
  - `writer` から `readonly` への fallback edge が、same semantic lineage の後段候補であることを明示する
- `@`
  - 「これは main clause ではなく metadata attachment である」という companion notation 上の印

重要なのは、`lineage` が「global lineage object」ではなく、**fallback edge に付く注記**として読まれている点である。
また、`@` を含むこの表記自体も **final token 決定ではない**。

## 7. その他、読み違えやすい点

### current L2 の examples は final grammar ではない

Phase 1〜3 の abstract に出てくるコード風の記法は、現在は **companion notation / representative examples** である。
「もう parser grammar が確定している」という意味ではない。

### `payload_core` などは helper / artifact 用語

`payload_core`、`bundle_context`、`detached_noncore` などは、detached validation loop の compare / emit / save を説明するための helper terminology であり、言語コアの syntax token ではない。

### `Reject` は option-local miss そのものではない

`admit` miss や `lease-expired` は option-local skip reason であり、request 全体の result としての `Reject` とは別である。
current L2 では、chain が success を返さずに尽きたときに request-level `Reject` を立てる。
