# 77 — current L2 stage 1 parser smoke family working set

## 目的

この文書は、`specs/examples/76-current-l2-stage1-parser-opaque-slot-carrier-and-bridge-api.md` で

- parser-side carrier の第一候補を `decl_guard_slot` とする
- thin lowering bridge を option-level structural transfer として読む

という current judgment を前提に、
**actual parser spike の stage 1 smoke をどの fixture family まで接続するのが最小で、かつ stage split を壊さないか**
を比較する。

ここで固定するのは final parser test plan ではない。
固定するのは、stage 1 actualization 前に docs-only で持つべき最小 working set だけである。

## 前提

- current L2 の core semantics は変更しない。
- stage 1 は chain / declaration structural floor に留める。
- declaration-side guard slot は opaque attached slot に留める。
- bridge は option-level structural transfer であり、parsed fact を stage 3 から先取りしない。
- `admit` predicate parse / request-local clause parse / `perform` parse は stage 1 non-goal である。

## current source anchor

### `e4-malformed-lineage`

- declaration structural floor が主題
- `OptionDecl.lease = "live"`、`admit = null`
- malformed lineage edge mismatch が static stop として見える

### `e7-write-fallback-after-expiry`

- valid chain declaration と mixed `lease` carrier が主題
- `lease = "expired"` と `lease = "live"` が coexist する
- `admit = null` のまま複数 option / 複数 edge を持つ

### `e3-option-admit-chain`

- option-local `admit` predicate node が主題
- stage 1 non-goal である request / admissibility cluster の pressure source になる

この 3 fixture から current source-backed に読めることは次である。

1. `e4` は declaration structural floor と malformed static stop の最小 contrast を与える
2. `e7` は valid chain declaration と mixed `lease` anchor を与える
3. `e3` は stage 1 ではまだ parse しない `admit` predicate node を含む

## 比較する working-set 案

### 案 1. `e4` 単独

stage 1 smoke を `e4-malformed-lineage` だけで始める。

#### 利点

- 最小の single-fixture smoke で済む。
- malformed lineage edge mismatch を structural floor として見やすい。

#### 欠点

- valid chain declaration の path が見えない。
- mixed `lease` anchor を実地で確認できない。
- option-level bridge が `OptionDecl.lease` を正しく transfer する最小 contrast が弱い。

### 案 2. `e4` + `e7`

stage 1 smoke を

- `e4-malformed-lineage`
- `e7-write-fallback-after-expiry`

の two-fixture pair で始める。

#### 利点

- malformed / valid の declaration structural contrast を同時に持てる。
- `e7` で mixed `lease` carrier を見られるため、`decl_guard_slot -> OptionDecl.lease` の bridge を narrow に確認しやすい。
- どちらも `admit = null` なので stage 1 non-goal を踏み越えにくい。

#### 欠点

- single-fixture よりは少し広い。
- request / admissibility cluster の contrast はまだ別途必要になる。

### 案 3. `e4` + `e7` + `e3`

stage 1 smoke に `e3-option-admit-chain` も含める。

#### 利点

- fixture-side `OptionDecl.admit` field を含む contrast まで早く見られる。
- stage 1 non-goal を explicit regression として近くに置ける。

#### 欠点

- stage 1 smoke に `admit` predicate node を入れることで、request / admissibility cluster を先食いしやすい。
- stage 1 parser spike の failure mode が「まだ非対応」なのか「bridge が placeholder だけ埋める」のかを混同しやすい。
- smoke family の目的が structural floor から boundary policing へ広がりすぎる。

## 比較

### stage 1 working set として narrow か

- `e4` 単独は narrow だが contrast が足りない。
- `e4` + `e7` は malformed / valid と mixed `lease` を最小で持てる。
- `e4` + `e7` + `e3` は contrast は増えるが stage 1 non-goal を近づけすぎる。

### `decl_guard_slot -> OptionDecl.lease` bridge を確認しやすいか

- `e4` 単独では `lease = "live"` しか見えない。
- `e7` を足すと `expired` / `live` の mixed anchor を見られる。
- `e3` は `admit` contrast としては useful だが、bridge そのものより non-goal policing が前面に出る。

### stage split を保てるか

- `e4` + `e7` は `admit = null` の pair なので stage 1 declaration structural floor に留めやすい。
- `e3` を active smoke に入れると、「`admit` を parse しないが placeholder は持つ」整理を実装前から複雑にしやすい。

## current judgment

current repo の next narrow step として、stage 1 parser spike の最小 smoke family は
**`e4-malformed-lineage` + `e7-write-fallback-after-expiry`**
の two-fixture pair にするのが最も自然である。

この judgment が意味するのは次である。

1. `e4` は malformed declaration structural floor の smoke anchor
2. `e7` は valid chain declaration と mixed `lease` anchor の smoke anchor
3. `e3` は stage 1 active smoke に入れず、request / admissibility cluster の later-stage contrast reference に残す

## current stage でまだ決めないこと

- actual parser spike の test file 配置
- smoke を parser-side AST compare にするか、fixture-side lowering compare にするかの exact harness shape
- `e3` を non-goal regression としてどの段で active smoke に昇格させるか
- stage 1 の smoke を `e4` / `e7` 以外へ広げる条件

## current meaning

- stage 1 smoke family は malformed / valid の declaration structural contrast と mixed `lease` anchor だけを見る
- `admit` predicate node を持つ fixture は stage 1 active smoke に入れない
- これにより stage 1 parser spike の working set を narrow のまま保ちつつ、bridge actualization に必要な contrast だけを確保できる
