# 76 — current L2 stage 1 parser opaque slot carrier and bridge API

## 目的

この文書は、`specs/examples/75-current-l2-stage1-parser-guard-slot-handoff.md` で固定した

- parser-side opaque slot carrier と fixture-side carrier を同一視しない
- `OptionDecl.lease` への thin lowering bridge を compatibility anchor に使う

という current judgment を前提に、
**stage 1 parser-side carrier の naming candidate と thin lowering bridge の private API surface をどこまで narrow に決めるのが自然か**
を比較する。

ここで固定するのは final parser API ではない。
固定するのは、actual parser spike の第 1 段を docs-only で誤配線しないための
最小 naming / handoff boundary だけである。

## 前提

- current L2 の core semantics は変更しない。
- stage 1 declaration-side guard slot は opaque attached slot に留める。
- predicate fragment parse / well-formedness は stage 3 以降へ残す。
- current parser-free AST fixture schema の `OptionDecl.lease` は compatibility anchor に留める。
- public `lib.rs` / `harness.rs` API は増やさない。
- final parser grammar、final parser-side AST schema、final parser API は OPEN のままである。

## current source anchor

### fixture-side anchor

current parser-free fixture schema では `OptionDecl` が少なくとも次を持つ。

- `target`
- `capability`
- `lease`
- `admit`

ここで `lease` は fixture-side companion carrier であり、current corpus では
`"live"` / `"expired"` のような narrow value を持つ。

### previous handoff judgment

`specs/examples/75-current-l2-stage1-parser-guard-slot-handoff.md` の current judgment は次である。

1. parser-side carrier と fixture-side carrier は同一視しない
2. thin lowering bridge を介して `OptionDecl.lease` へ narrow lowering する
3. exact carrier 名と actual bridge API surface は次段へ残す

### helper naming anchor

current repo の detached transform helper は、slot 単位や program-wide ではなく、
**責務単位の whole-carrier transform** として切っている。

例:

- `build_detached_bundle_artifact(...)`
- `build_detached_aggregate_artifact(...)`

この naming / API cut は、

- caller に hidden elaboration を見せにくい
- narrow private API に留めやすい
- public surface を早く既成事実化しにくい

という current helper boundary と整合する。

## 比較する naming candidate

### 案 1. parser-side carrier を `lease` / `lease_slot` に寄せる

parser-side option declaration に、`lease` または `lease_slot` のような field 名を付ける。

#### 利点

- fixture-side `OptionDecl.lease` と見た目が近く、直感的には短い。
- stage 1 実装だけを考えると naming が最も簡単である。

#### 欠点

- parser-side opaque slot と fixture-side compatibility anchor の区別が薄れる。
- later stage で predicate fragment parse を導入するとき、`lease` という semantics-loaded name が drift source になる。
- stage 1 ではまだ predicate meaning を読まないのに、name が meaning を先取りしやすい。

### 案 2. parser-side carrier を `guard_slot` にする

parser-side option declaration に、generic な `guard_slot` field を付ける。

#### 利点

- `lease` よりは fixture-side anchor との混線が少ない。
- stage 1 の opaque attached slot 読みとも矛盾しない。

#### 欠点

- declaration-side なのか request-side なのかが name だけでは見えにくい。
- later stage で request / admissibility cluster にも guard-like carrier が入ると generic すぎる。

### 案 3. parser-side carrier を `decl_guard_slot` にする

parser-side option declaration に、`decl_guard_slot` のような
**declaration-side / stage-local / still-opaque**
であることが読める field 名を付ける。

#### 利点

- fixture-side `lease` と明確に区別できる。
- request-side guard や `admit` predicate node とも混線しにくい。
- stage 1 で attached slot の structural boundary だけを見る current docs-only judgment と整合する。

#### 欠点

- `lease` よりは少し長い。
- final parser-side schema にもそのまま残るとは限らないため、temporary feeling がある。

## 比較する bridge API surface

### 案 A. slot-only bridge

```text
lower_decl_guard_slot_to_fixture_lease(slot) -> fixture_lease
```

#### 利点

- 最小の pure transform に見える。
- `lease` 変換だけを切り出せる。

#### 欠点

- caller が option declaration の残り field を別管理しなければならず、handoff point が細かく割れやすい。
- slot 単位 transform が、later stage で hidden parsing / normalization を足し込みやすい。
- current helper boundary の「whole-carrier transform」パターンと非対称になる。

### 案 B. option-level bridge

```text
lower_stage1_option_decl_to_fixture_option(option_decl) -> fixture_option_decl
```

#### 利点

- parser-side option declaration を 1 つの structural carrier として渡せる。
- `decl_guard_slot` を含む option declaration 全体の ownership が明確になる。
- thin lowering bridge であっても、どこまでが structural transfer でどこから先が predicate parse かを切りやすい。
- current detached helper の責務単位 transform とも整合する。

#### 欠点

- slot-only bridge よりは API の粒度が少し大きい。
- stage 1 parser-side option declaration の local temporary shape を narrow に決める必要がある。

### 案 C. program-level bridge

```text
lower_stage1_program_to_fixture_program(program) -> fixture_program
```

#### 利点

- caller の handoff は 1 箇所にまとまる。

#### 欠点

- stage 1 declaration structural floor に対して粒度が大きすぎる。
- chain / request / try-rollback cluster までまとめて early freeze しやすい。
- current next step では unnecessary に広い。

## 比較

### stage 1 の docs-only cut を保てるか

- naming
  - `lease` / `lease_slot` は fixture-side anchor と近すぎる。
  - `guard_slot` は generic すぎる。
  - `decl_guard_slot` は declaration-side opaque slot を最もそのまま表す。
- bridge
  - slot-only bridge は局所的すぎて hidden elaboration point を作りやすい。
  - option-level bridge は structural-only cut を保ちやすい。
  - program-level bridge は広すぎる。

### fixture-side compatibility anchor を保てるか

- `decl_guard_slot` を parser-side name にしておけば、fixture-side `OptionDecl.lease` が compatibility anchor だと明確に読める。
- option-level bridge を使うと、`OptionDecl.lease` への transfer を option declaration 1 単位で閉じられる。

### hidden parsing / normalization を避けられるか

- slot-only bridge では later stage で slot 単位 helper に predicate normalization を足しやすい。
- option-level bridge なら、
  `stage1 option declaration -> fixture option declaration`
  という structural transfer に責務を固定しやすい。
- program-level bridge は責務が大きすぎ、hidden elaboration を招きやすい。

### current helper boundary と整合するか

- detached helper 群は current repo で whole-carrier transform を narrow private API として持っている。
- option-level bridge はこのパターンと一番よく揃う。
- slot-only bridge は粒度が細かすぎ、program-level bridge は粒度が粗すぎる。

## current judgment

current repo の next narrow step としては、次の組合せが最も自然である。

1. parser-side opaque slot carrier の field 名は **`decl_guard_slot`** を第一候補にする
2. thin lowering bridge の private API surface は **option-level bridge** として読む
3. したがって actual parser spike の docs-only handoff は、
   `stage1 option declaration -> fixture OptionDecl`
   の structural transfer として比較する

この judgment が意味するのは次である。

- `decl_guard_slot` は fixture-side `lease` ではない
- option-level bridge は predicate parse / normalization を行わない
- option-level bridge は stage 1 で実際に parse した declaration subset だけを transfer し、`OptionDecl` の stage-3-only field は parsed fact として materialize しない
- `OptionDecl.admit = null` のような値が必要な場合でも、それは fixture-side compatibility placeholder であり、stage 1 parser が predicate を読んだことを意味しない
- bridge は structural-only / compatibility-only であり、public API にも上げない

## current stage でまだ決めないこと

- `decl_guard_slot` field の内部 carrier が raw text / token slice / opaque leaf のどれか
- actual Rust function 名
- actual Rust type 名
- bridge helper の配置が `examples/` / `tests/support/` / parser spike private module のどこになるか
- later stage で `decl_guard_slot` を parser-side richer carrier へ widen するかどうか

## current meaning

- stage 1 parser-side guard slot naming は fixture-side `lease` と切り分けて読む
- bridge は slot-only helper ではなく option declaration 全体の structural transfer として扱う
- これにより stage 1 handoff は narrow のまま保ちつつ、later stage の predicate parse を hidden に先食いしない
