# 108 — current L2 stage 3 request contract subset compare cut

## 目的

この文書は、`specs/examples/107-current-l2-stage3-suite-reopen-conditions.md` で
fixed two-slot suite bridge を fixture-side full request contract compare へ
**narrow 条件つきで reopen してよい**
と整理したことを前提に、
**その first actualization cut をどこに置くのが最小か**
を比較する。

ここで固定するのは final parser grammar でも full request node compare でもない。
固定するのは、current phase の stage 3 later branch で

- source-side helper output は two-slot carrier に留め
- fixture-side compare は contract subset に留め
- request head parse は still later に残す

という条件の下での **first actualization cut** だけである。

## 前提

- current L2 の core semantics、fixture schema、runtime semantics は変更しない。
- fixed two-slot suite bridge helper は actualize 済みである。
- source-side helper output は `Stage3RequestClauseSuite { require_fragment_text, ensure_fragment_text }` に留める。
- existing isolated predicate fragment helper は、fixture-side `contract.require` / `contract.ensure` から individual predicate fragment を load できる。
- request head kind / op / target / chain_ref の source-side parse は still later stage に残す。

## current issue

current suite spike tests は already 次を行えている。

- source-side suite bridge から `require_fragment_text` / `ensure_fragment_text` を抽出する
- individual clause fragment を parse する
- fixture-side individual predicate fragment と ad-hoc に compare する

しかし、この line はまだ
**fixture-side full request contract subset compare**
としては明示的に切れていない。

次段では、次のどこを first actualization cut に置くかを決める必要がある。

## 比較する 3 案

### 案 1. ad-hoc per-slot compare のまま留める

current tests のように、

- `require_fragment_text`
- `ensure_fragment_text`

を個別に parse し、fixture-side individual predicate fragment と compare する。

#### 利点

- 追加 helper は最小である。
- 既存テストを少し整えるだけで済む。

#### 欠点

- `contract subset compare` の boundary が docs / tests 上で still implicit である。
- `fixed two-slot suite bridge` と `fixture-side contract subset` の接続が reusable な compare cut にならない。
- 次段 actualization のたびに ad-hoc wiring が増えやすい。

### 案 2. dedicated helper-local contract subset carrier を切る

fixture-side expected shape を dedicated helper-local carrier として切る。

```text
Stage3RequestContractSubset {
  require_fragment: Option<Stage3PredicateFragment>,
  ensure_fragment: Option<Stage3PredicateFragment>,
}
```

この案では、

- fixture-side `PerformOn` / `PerformVia` の contract subset だけを読む
- request head kind / op / target / chain_ref は carrier に入れない
- source-side suite bridge の two-slot carrier と 1 対 1 の compare line を切る

#### 利点

- `fixed two-slot suite bridge` と `fixture-side contract subset compare` の接点が明示になる。
- request head parse を still later に残せる。
- full request node compare へ既成事実化せずに compare family を 1 段前へ出せる。

#### 欠点

- helper-local expected carrier が 1 つ増える。
- compare helper の placement を慎重に切らないと、fixture-side full request node compare へ滑りやすい。

### 案 3. full request node compare を first cut にする

fixture-side `PerformOn` / `PerformVia` subset 全体、あるいは source-side request head kind まで含める。

#### 利点

- compare の見通しは一見よい。

#### 欠点

- request head parse pressure が強い。
- current reopen 条件を破りやすい。
- stage 3 later branch の narrow line を越える。

## 比較

### stage 3 reopen 条件との整合

- 案 1 は request head parse を増やさないが、reopen した compare line 自体が still implicit のまま残る。
- 案 2 は request head parse を still later に残したまま、contract subset compare を explicit に切れる。
- 案 3 は reopen 条件を越える。

### helper family の責務分離

- current phase では、
  - source-side suite bridge extraction
  - fixture-side predicate fragment load
  - contract subset compare
  を混ぜすぎない方がよい。
- 案 2 は dedicated helper-local carrier を 1 つ足すだけなので、family の分離をまだ保てる。
- 案 1 は family が implicit なまま、案 3 は family をまとめすぎる。

### next actualization の素直さ

- 案 1 は current test の延長としては楽だが、compare cut 自体の進展が小さい。
- 案 2 は helper-local / test-only first tranche として actualize しやすい。
- 案 3 は actualization するほど later-stage request head parse を既成事実化しやすい。

## current judgment

current repo の next narrow actualization cut としては、
**案 2. dedicated helper-local contract subset carrier を切る**
のが最も自然である。

つまり current phase では、

- source-side helper output は `Stage3RequestClauseSuite`
- fixture-side expected shape は `Stage3RequestContractSubset`
- compare 対象は
  - `require_fragment`
  - `ensure_fragment`
  の 2 slot だけ

に留めるのが最小である。

## なぜこれが最小か

- request head parse を still later に残せる。
- fixed two-slot suite bridge と fixture-side contract subset compare の接点を explicit にできる。
- full request node compare を既成事実化せずに、Phase 3 の structural compare line を一段進められる。

## current stage でまだやらないこと

- source-side request head kind / op / target / chain_ref parse
- fixture-side full request node compare
- public parser API 化
- typed diagnostics carrier

## next narrow step

次は、
**`Stage3RequestContractSubset` 相当の helper-local / test-only first tranche を actualize する**
のが自然である。
