# 34 — current L2 static reason code entry criteria

## 目的

この文書は、current L2 parser-free PoC で `expected_static.checked_reasons` を導入した次段として、
**どの static gate reason cluster なら typed reason code に進めてよいか**
を narrow に整理する。

ここで固定するのは final reason taxonomy ではない。
固定するのは、

- まだ string carrier に留めるべき範囲
- code 化してよい cluster の条件
- code 化するなら flat string ではなく parametric shape を選ぶべきこと

だけである。

## current code anchor

current `static_gate_detailed()` が actual source として emit する reason pattern は、少なくとも次である。

1. declaration duplicate 由来 reason
2. `missing option declaration for chain head {head} at {scope}`
3. `missing predecessor option {option} at {scope}`
4. `missing successor option {option} at {scope}`
5. `missing lineage assertion for {predecessor} -> {successor}`
6. `lineage assertion does not describe {predecessor} -> {successor}`
7. `declared access target is missing for {predecessor} -> {successor}`
8. `declared access target mismatch between {predecessor} and {successor}`
9. `capability strengthens from {predecessor_capability} to {successor_capability}`

## 比較対象

### 案 1. すぐ typed reason code へ全面移行する

すべての static gate reason を早期に code 化する。

#### 利点

- wording drift を大きく減らせる
- checker / detached artifact / theorem prover の contract を揃えやすい

#### 欠点

- duplicate reason のように declaration index 内部に依存する cluster まで premature に凍る
- parameter slot の shape を早く固定しすぎる
- current L2 では malformed / underdeclared 以外の future cluster を先取りしやすい

### 案 2. `checked_reasons` を bridge にしつつ、stable cluster だけ code 化する

current machine-check は string `checked_reasons` を維持しつつ、
reason code に向く cluster を inventory 化する。

#### 利点

- current harness / fixture authoring / detached artifact loop を壊さない
- code 化に向く cluster だけを段階的に切り出せる
- parameter slot を持つ code shape を先に設計できる

#### 欠点

- しばらく string と code の二重管理になる
- migration timing の discipline が要る

### 案 3. 当面 string carrier に留める

`checked_reasons` を維持し、reason code は後段へ送る。

#### 利点

- taxonomy を急がない
- current L2 の narrow progression を保ちやすい

#### 欠点

- wording drift を減らす効果は限定的
- theorem prover / checker 契約に上げる際の bridge が弱い

## current judgment

current L2 の next narrow step として自然なのは **案 2** である。

理由は次の通り。

1. `checked_reasons` はすでに additive optional carrier として導入済みである
2. ただちに全面 code 化するには cluster ごとの parameter slot がまだ粗い
3. しかし code 化に向く cluster の inventory は先に持てる

## code 化してよい cluster の条件

typed reason code に進めてよいのは、少なくとも次を満たす cluster に限る。

1. current semantics で意味が安定している
2. user-facing prose と切り離しても意味が保てる
3. parameter slot が明確である
4. detached artifact / checker / theorem prover のどこでも同じ skeleton を使える

## current candidate cluster

### code 化に向く

- `missing_lineage_assertion`
  - slots: `predecessor`, `successor`
- `lineage_assertion_edge_mismatch`
  - slots: `predecessor`, `successor`
- `declared_target_missing`
  - slots: `predecessor`, `successor`
- `declared_target_mismatch`
  - slots: `predecessor`, `successor`
- `capability_strengthens`
  - slots: `from_capability`, `to_capability`
- `missing_chain_head_option`
  - slots: `head`, `scope`
- `missing_predecessor_option`
  - slots: `option`, `scope`
- `missing_successor_option`
  - slots: `option`, `scope`

### まだ code 化を急がない

- declaration duplicate 由来 reason

これは declaration index 内部構造との結びつきが強く、current helper 実装の wording / grouping を早く固定しすぎやすい。

## flat string code を採らない理由

typed reason code に進むなら、単なる enum string ではなく
**kind + parameter slots**
の shape を採る方が自然である。

例:

```text
{
  kind: "missing_lineage_assertion",
  predecessor: "primary",
  successor: "mirror"
}
```

これにより、

- checker の fail-closed compare
- detached artifact の stable contract
- theorem prover 側の relation

を同じ cluster 名でつなぎやすい。

## current L2 でまだ決めないもの

- final code schema の exact serialization
- duplicate reason cluster の code 化
- detached static gate artifact に `reason_codes` mirror をいつ足すか
- `checked_reasons` をいつ deprecated にするか

## current judgment

- immediate 全面 code 化はまだ早い
- `checked_reasons` は current bridge として維持する
- 次段では code 化に向く stable cluster のみ inventory 化し、parametric shape を比較する
