# 110 — current L2 stage 3 request contract subset widening guard

## 目的

この文書は、`specs/examples/109-current-l2-stage3-request-contract-subset-first-tranche-actualization.md` で
`Stage3RequestContractSubset` helper-local / test-only first tranche が source-backed に通ったことを前提に、
**request head metadata を still later に残したまま、contract-only compare surface をどこまで widening してよいか**
を比較する。

ここで固定するのは full request node compare でも request head parse でもない。
固定するのは、

- `contract.require`
- `contract.ensure`

だけを見る current compare family を、この時点でまだ widening すべきかどうかの **guard judgment** である。

## 前提

- current L2 の core semantics、fixture schema、runtime semantics は変更しない。
- source-side helper output は still `Stage3RequestClauseSuite { require_fragment_text, ensure_fragment_text }` に留める。
- fixture-side expected carrier は `Stage3RequestContractSubset` に留める。
- request head kind / op / target / chain_ref parse は still later stage に残す。
- current fixture corpus では、`PerformOn` / `PerformVia` の `contract.require` / `contract.ensure` array に 2 個以上の predicate row を持つ fixture はまだない。
- single predicate fragment の内部では、explicit `and` を既存 predicate fragment helper で already 表現できる。

## current issue

spec109 actualization により、

- same source-side suite carrier を
- `PerformOn` / `PerformVia`

の両 fixture に対して contract-only compare できる line は source-backed になった。

ここで次に迷うのは、

1. current `0-or-1` clause guard を維持するか
2. fixture-side carrier を clause row list へ widening するか
3. source-side / fixture-side の両 carrier をまとめて row-list family へ widening するか

である。

## 比較する 3 案

### 案 1. current `Stage3RequestContractSubset` を 0-or-1 guard のまま維持する

```text
Stage3RequestContractSubset {
  require_fragment: Option<Stage3PredicateFragment>,
  ensure_fragment: Option<Stage3PredicateFragment>,
}
```

2 個以上の predicate row を持つ clause array は、引き続き
`outside stage 3 first tranche`
として fail-closed にする。

#### 利点

- source-side `Stage3RequestClauseSuite` と compare shape が still 1 対 1 に保たれる。
- request head metadata を still later に残せる。
- current fixture corpus と current suite helper の実力に対して overfit しない。
- conjunction 自体は single `Stage3PredicateFragment::And` で already 表せるため、表現力が直ちに不足するわけではない。

#### 欠点

- clause array の multi-row shape を future compare へまだ送れない。

### 案 2. fixture-side expected carrier だけを clause row list へ widening する

```text
Stage3RequestContractSubsetRows {
  require_fragments: Vec<Stage3PredicateFragment>,
  ensure_fragments: Vec<Stage3PredicateFragment>,
}
```

source-side helper output は still one-slot text のままにする。

#### 利点

- fixture-side schema が current JSON array shape に closer になる。

#### 欠点

- source-side helper output が still one-slot text なので compare surface が asymmetric になる。
- actual source-backed fixture corpus に multi-row anchor が無い段階で expected carrier だけ先に太る。
- current questionが contract-only compare surface の widening guard であるにもかかわらず、fixture-side widening だけが先行して family が歪む。

### 案 3. source-side / fixture-side をまとめて row-list family へ widening する

source-side helper も clause row list を持つように widen し、fixture-side expected carrier も row-list へ揃える。

#### 利点

- shape の対称性自体は高い。

#### 欠点

- current suite helper は fixed two-slot suite bridge として切っており、row-list family へ widen すると first tranche の設計根拠が変わる。
- current fixture corpus に multi-row clause array anchor がない。
- request head parse は入れなくても、source-side structural widening の pressure が急に上がる。
- Phase 3 later branch の narrow line を越えやすい。

## 比較

### current fixture corpus との整合

- current fixture corpus には multi-row clause array がない。
- したがって row-list widening は current source-backed evidence ではなく、future possibility を先取りする比重が強い。

### current source-side helper との整合

- source-side `Stage3RequestClauseSuite` は
  - `require_fragment_text`
  - `ensure_fragment_text`
  の fixed two-slot carrier である。
- ここから fixture-side expected carrier だけ row-list へ widen すると compare family が asymmetric になる。
- source-sideも同時 widening すると、今度は spec100 / 101 / 108 / 109 までで積んだ fixed two-slot suite bridge family 自体を早く崩す。

### 表現力との関係

- current predicate fragment helper は `and` tree を already 表せる。
- したがって「複数条件を一切表現できない」わけではない。
- multi-row clause array を今すぐ compare しないことは、current L2 の表現力の直ちの欠損ではなく、**compare surface widening を later に送る**判断である。

## current judgment

current repo の next narrow step としては、
**案 1. current `Stage3RequestContractSubset` を 0-or-1 guard のまま維持する**
のが最も自然である。

つまり current phase では、

- request head metadata は still later
- contract-only compare surface も still two-slot / 0-or-1 guard
- multi-row clause array widening は future comparison

に留める。

## なぜこれが最小か

- current fixture corpus に multi-row anchor がない。
- source-side fixed two-slot suite bridge と compare shape を still 1 対 1 に保てる。
- explicit `and` による grouped predicate fragment は already 比較可能なので、表現力と compare widening を混同しないで済む。
- request head parse pressure と source-side helper widening pressureを同時に上げずに済む。

## current stage でまだやらないこと

- clause row list carrier への widening
- source-side suite bridge の row-list 化
- request head kind / op / target / chain_ref parse
- fixture-side full request node compare
- public parser API 化

## next narrow step

次段では、

1. future widening を許す entry criteria を source-side suite bridge 側から先に切るか
2. request head metadata still deferred のまま contract-only compare family をここで一旦凍結し、別 Phase 3 subline に戻るか

を narrow に比較するのが自然である。
