# 83 — current L2 stage 3 admit-slot branch comparison

## 目的

この文書は、`specs/examples/80-current-l2-stage1-parser-spike-first-tranche-actualization.md` と
`specs/examples/82-current-l2-stage1-parser-spike-malformed-source-first-tranche-actualization.md`
で stage 1 parser spike の structural floor が actualize 済みであることを前提に、
**request / admissibility cluster を stage 3 として進めるとき、その最初の sub-cut を何にするのが自然か**
を narrow に比較する。

ここで固定するのは final parser grammar ではない。
固定するのは、`e3-option-admit-chain` を later-stage contrast reference として読むときの
**next narrow semantic cut** だけである。

## 前提

- current L2 の core semantics は変更しない。
- stage 1 parser spike は declaration / chain structural floor と malformed-source pair に留める。
- declaration-side `lease` guard slot は opaque attached slot として保持し、predicate fragment parse はまだ later stage に残す。
- `specs/examples/29-current-l2-first-parser-subset-inventory.md` の cluster inventory は維持する。

## current source anchor

- `specs/examples/29-current-l2-first-parser-subset-inventory.md`
  - option-local `admit` は first parser cut 候補 cluster に入っている
- `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md`
  - option-local `admit` は request-local clause attachment と別 cluster である
- `specs/examples/74-current-l2-stage1-parser-spike-entry-criteria.md`
  - stage 1 non-goal に option-local `admit` と `perform` / request cluster が明示されている
- `crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json`
  - option-local `admit` と `PerformVia` / `require` を同居させた current contrast anchor

## 比較する 3 案

### 案 1. `e3` を request / admissibility cluster ごと一気に送る

次段で一気に次を受ける。

- option-local `admit`
- `PerformVia`
- request-local `require`

#### 利点

- `e3` の representative shape を一度に parser spike へ近づけられる。
- parser-free fixture と closer に見える。

#### 欠点

- option-local admissibility と request / execution head が同時に入る。
- current grammar 未決の lexical pressure が急に増える。
- stage 3 の内部 sub-cut を作らず、一気に lexical pressure を持ち込みやすい。

### 案 2. stage 3 の最初の sub-cut として declaration-side `admit` attached slot だけを先に切る

次段では option declaration 側にだけ

- `admit SLOT`

を opaque attached slot として足し、
`PerformVia` / request-local `require` / `ensure` はまだ later stage に残す。

#### 利点

- `lease` guard slot と対称な declaration-side expansion として読める。
- option-local admissibility と request-local clause の役割分担を崩さない。
- `e3` を full-program parse ではなく option / chain subset reference として使える。
- predicate fragment parse をまだ later stage に残せる。

#### 欠点

- stage 3 側の helper / compare line には second attached slot を足す必要がある。
- `e3` の `PerformVia` 部分はまだ compare から外す必要がある。

### 案 3. stage 3 の最初の sub-cut を `PerformVia` / request-local clause に寄せ、`admit` は後に回す

#### 利点

- request head / request-local contract は inventory 上も強い cluster である。

#### 欠点

- `e3` の main contrast は option-local admissibility miss による chain continuation であり、`admit` を後回しにすると contrast の中心がずれる。
- declaration-side attached slot と request-side clause の分離が弱くなる。
- `admit` を later に送る理由が source-backed に弱い。

## 比較

### current stage order との整合

- 案 1 は stage 3 の内部 sub-cut を飛ばしやすい。
- 案 2 は stage 1 の declaration-side slot line と対称な stage 3 branch として最も自然である。
- 案 3 は request cluster を先に切るが、`e3` contrast の中心から外れやすい。

### lexical freeze pressure

- 案 1 が最も高い。
- 案 2 は `admit` attached slot の surface だけを見ればよいので低い。
- 案 3 は `perform` / `require` の concrete syntax を早く固定しやすい。

### `e3` anchor の使い方

- 案 1 は `e3` を full-program reference にする。
- 案 2 は `e3` を option / chain subset reference に留める。
- 案 3 は `e3` から option-local admissibility の中心を外しやすい。

## current judgment

current repo の next narrow step としては、**案 2** が最も自然である。

1. stage 3 request / admissibility cluster の最初の sub-cutは declaration-side `admit` attached slot を先に切る
2. `admit` の内部は current stage でも opaque attached slot に留める
3. `PerformVia` / request-local `require` / `ensure` は still later stage に残す
4. `e3` は full-program compare ではなく、option declaration + chain subset の contrast reference として使う

## なぜこれが最小か

- stage 1 の `lease` guard slot と対称な attached-slot extension として読める。
- option-local admissibility と request-local contract の分離を保てる。
- `e3` の semantic center である option-local admissibility miss を、request parse に先行して扱える。
- `perform` / request grammar を早く固定しない。

## current stage でまだやらないこと

- `PerformVia` line の parse
- request-local `require` / `ensure`
- `admit` predicate fragment の internal parse
- full `e3` program compare
- public parser API

## next narrow step

この comparison を actualize するなら、次が最小である。

1. option declaration に optional `admit` attached slot を足す
2. `e3` 由来の inline text を option / chain subset compare に留める
3. current `PerformVia` line は parser helper の input から外す
