# 26 — current L2 detached aggregate compare helper

## 目的

この文書は、current L2 parser-free PoC の detached validation loop で、

- aggregate detached artifact を 2 本保存したあとに
- `summary_core` だけを narrow に比較し
- `aggregate_context` と `detached_noncore` は reference-only に留める

ための non-production helper 境界を整理する。

ここで固定するのは production compare API ではない。
固定するのは、`bundle_failure_kind_counts_scope = "migrated-kinds-only"` を前提にした
**aggregate compare helper の最小 operational cut** だけである。

## 適用範囲

- current L2 の core semantics は変えない
- aggregate detached artifact の actual narrow cut は `specs/examples/25-current-l2-detached-aggregate-emitter-sketch.md` に従う
- `bundle_failure_kind_counts` は full histogram ではなく partial histogram として読む
- `must_explain` や long-form explanation は compare core に上げない
- final exporter API、final path policy、profile-aware aggregate compare は固定しない

## current helper の位置づけ

current detached validation loop で aggregate compare を担う actual narrow cut は、
`scripts/current_l2_diff_detached_aggregates.py` を non-production helper として置くところまでである。

この helper は次を行う。

- 2 本の aggregate artifact JSON を読む
- `summary_core` を exact-compare core として比較する
- `aggregate_context` と `detached_noncore` は reference-only differences として表示する

この helper は次を行わない。

- `harness.rs` / `lib.rs` の public compare API を増やすこと
- bundle failure summary を aggregate core に再掲すること
- full histogram 前提の比較を既成事実化すること
- final machine-readable compare contract を固定すること

## exact-compare core

aggregate compare helper が exact-compare core として扱うのは、`summary_core` の次の field に限る。

- `total_bundles`
- `runtime_bundles`
- `static_only_bundles`
- `passed`
- `failed`
- `bundle_failure_kind_counts_scope`
- `bundle_failure_kind_counts`

`bundle_failure_kind_counts_scope` を core に含める理由は、current actual sketch が partial histogram であり、その scope marker を外すと full histogram と誤読しやすくなるからである。

## reference-only sections

aggregate compare helper では、次は reference-only に留める。

- `aggregate_context`
- `detached_noncore`

### `aggregate_context`

- `directory_path`
- `aggregate_scope`

これは compare 対象 run の provenance を示すために有益だが、current L2 の typed aggregate core そのものではない。

### `detached_noncore`

- `discovery_failures`
- `bundle_failures`
- `host_plan_coverage_failures`

これらは coarse summary の補助や explanation に有益だが、`bundle_failure_kind_counts` と current list anchor の coexistence を保つため、typed aggregate core には上げない。

## current wrapper との接続

current detached validation loop の tiny wrapper は、aggregate compare 側では少なくとも次を支えてよい。

- `compare-aggregates`
  - 2 run label を受け取る
  - `target/current-l2-detached/aggregates/<run-label>/batch-summary.detached.json` から path を導出する
  - aggregate compare helper を呼ぶ
- `compare-fixture-aggregates`
  - 2 fixture を temporary single-fixture directory に複製する
  - `summary_core` compare 用の aggregate artifact を 2 本だけ emit する
  - noisy な full-vs-single contrast を経由せず single-fixture aggregate 同士を比べる

この wrapper は convenience discovery を与えるだけであり、final compare input discovery policy を固定しない。

## なぜ explicit path compare だけにしないか

aggregate compare helper 自体は explicit path で呼べば十分である。
ただし detached validation loop の current non-production phase では、

- 1 directory を emit する
- run label ごとに保存する
- その 2 run をすぐ比較する

という往復が多い。
このため wrapper 側で run label から aggregate artifact path を導出する最小 convenience は許してよい。

一方で、これは production CLI ではないので、

- arbitrary discovery rule の一般化
- profile-aware aggregate compare
- cross-root compare

までは持ち込まない。

## OPEN に残すもの

- aggregate compare helper を `harness.rs` / `lib.rs` の public behavior に上げるか
- compare input discovery を run label convenience 以上に formalize するか
- `bundle_failure_kind_counts` を row list のまま compare し続けるか
- profile / named profile summary compare をどの timing で足すか
- current list anchor と typed count field の coexistence をいつまで保つか

## current judgment

- aggregate compare helper は `summary_core` だけを exact-compare してよい
- `aggregate_context` と `detached_noncore` は reference-only でよい
- `bundle_failure_kind_counts_scope = "migrated-kinds-only"` は exact-compare core に残す
- run label から aggregate artifact path を導出する tiny wrapper は current detached validation loop の convenience として許容する
- ただし final compare API、final path policy、profile-aware aggregate compare はまだ固定しない
