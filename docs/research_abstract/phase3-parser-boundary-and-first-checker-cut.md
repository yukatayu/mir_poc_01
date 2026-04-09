# Phase 3 要約 — parser boundary と first checker cut

## 何をした phase か

Phase 3 は、
**final parser grammar を決めずに、parser boundary と first checker cut を staged spike として narrow に切る phase**
である。

ここでは public parser / public checker を作るのではなく、
private helper と focused compare family で、どこまで local / structural floor を持ち上げられるかを見た。

## 3 つの staged spike

### stage 1 — chain / declaration structural floor

扱うもの:

- option declaration core
- explicit edge-row family
- edge-local lineage metadata
- declaration-side guard slot

例:

```text
option writer on doc capability write lease live

chain doc_ref = writer
  fallback readonly
    @ lineage(writer -> readonly)
```

ここで `@ lineage(...)` は fallback edge に付く edge-local metadata attachment であり、`lineage` が global object として独立に動くわけではない。

ここでは same-lineage / missing-option / capability strengthening の floor と reconnect させた。

### stage 2 — try / rollback structural floor

扱うもの:

- `try { ... } fallback { ... }`
- `atomic_cut`
- malformed pair と minimal valid smoke

例:

```text
try {
  perform write via doc_ref
  atomic_cut
} fallback {
  perform enqueue on retry_queue
}
```

ただし、`E21` / `E22` のような runtime/proof contrast は
still external boundary に残した。

### stage 3 — request / admissibility structural floor

扱うもの:

- declaration-side `admit`
- request-local `require` / `ensure`
- minimal predicate fragment
- multiline attachment
- fixed two-slot clause suite
- request contract subset

例:

```text
option writer on doc capability write lease live
  admit:
    user_can_write and lease_live

perform write via doc_ref
  require:
    user_can_write
  ensure:
    write_committed
```

ここでも full request parser や public API には上げず、
helper-local / test-only compare に留めた。

## Phase 3 で重要だった判断

### 1. final grammar を決めない

Phase 3 では parser を触るが、
**A2 / A1 の final lexical choice や reserved keyword を固定しない**。

### 2. local / structural floor に限定する

first checker cut に寄せるのは、

- malformed / underdeclared rejection
- same-lineage floor
- minimal capability strengthening prohibition
- clause attachment
- minimal predicate fragment well-formedness

のような local / structural / decidable 寄りのものだけである。

### 3. runtime / proof boundary は無理に持ち込まない

たとえば、

- `e19` は typed static reason family に残す
- `E21` / `E22` は runtime / proof boundary に残す

という freeze threshold を置いた。

## current checkpoint の意味

Phase 3 current tranche では、

- stage 1 / 2 / 3 の private staged spike
- reconnect subline の freeze threshold

まで整理できた。

その結果、current checkpoint では
**Phase 3 self-driven portion は一旦尽きた**
と読むのが自然になった。

つまり、これ以上いま自走で進めると、

- wording refinement に留まるか
- Phase 5 / 6 の public boundary を先取りするか
- runtime / proof boundary に踏み込みすぎるか

のどれかになりやすい。

## 次の phase へ渡したもの

- parser subset inventory
- first checker cut entry criteria
- private staged spike の representative evidence
- reopen が必要になったときの freeze line

このため、次の主線は

- Phase 2 maintenance tail
- Phase 4 shared-space boundary
- Phase 5 small decidable core inventory

へ移してよい。
