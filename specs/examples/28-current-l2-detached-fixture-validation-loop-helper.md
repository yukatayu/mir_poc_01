# 28 — current L2 detached fixture validation-loop helper

## 目的

この文書は、current L2 parser-free PoC の detached validation loop で、

- 1 fixture を bundle artifact として保存し
- 必要なら reference fixture と payload core を比較し
- さらに single-fixture aggregate smoke まで 1 command で回す

ための non-production helper 境界を整理する。

ここで固定するのは production CLI ではない。
固定するのは、fixture authoring / elaboration bottleneck を 1 段だけ軽くする
**fixture validation-loop helper の最小 operational cut** だけである。

## 適用範囲

- current L2 の core semantics は変えない
- bundle-first detached artifact、aggregate detached artifact、diff helper の current boundaryを再利用する
- `must_explain` や long-form explanation は compare target に上げない
- final exporter API、final storage/path policy、profile-aware fixture loop CLI は固定しない

## current helper の位置づけ

current detached validation loop で 1 fixture の反復を支える actual narrow cut は、
`scripts/current_l2_detached_loop.py` に `smoke-fixture` subcommand を足すところまでである。

この helper は次を行う。

1. target fixture を bundle artifact として emit する
2. optional な reference fixture があれば、その bundle artifact も emit して payload core を compare する
3. fixture 親 directory 全体の aggregate artifact を emit する
4. target fixture だけを一時 directory に複製した single-fixture aggregate artifact を emit する
5. full directory aggregate と single-fixture aggregate の `summary_core` を compare する

さらに current first tranche では、fixture 引数に

- explicit path
- current fixture directory に対する stem shorthand

を受けてよい。missing fixture は deep IO error に落とさず fail-fast に止める。

さらに current helper は、operational friction を下げるために次を支えてよい。

- fixture 引数は explicit path だけでなく fixture stem shorthand でも受けてよい
- `smoke-fixture` と `compare-fixtures` の run label は、明示されない場合 fixture stem から導出してよい
- missing fixture / missing fixture directory は emitter subprocess へ落とす前に fail-closed にしてよい
- noisy な full-vs-single aggregate contrast と fixture-to-fixture aggregate compare を分けるため、`compare-fixture-aggregates` subcommand を追加してよい

この helper は次を行わない。

- `harness.rs` / `lib.rs` の public helper API を増やすこと
- production exporter CLI を既成事実化すること
- fixture expectation や sidecar 内容を自動補完すること
- full aggregate API finalization を先取りすること

## path policy の最小 cut

current helper は、既存 detached loop と同じ current non-production default candidate を使う。

- artifact root
  - `target/current-l2-detached/`
- bundle artifact
  - `bundles/<run-label>/<fixture-stem>.detached.json`
- aggregate artifact
  - `aggregates/<run-label>/batch-summary.detached.json`

`smoke-fixture` は run label からさらに次を導出してよい。

- full directory aggregate
  - `<run-label>-full`
- single-fixture aggregate
  - `<run-label>-single`

これは convenience discovery であり、final path policy ではない。
run label 自体が省略されたときは、target fixture の stem を primary label に使ってよい。
reference fixture を併用するときは、reference 側もその stem を default label にしてよい。

## compare の最小 cut

### bundle compare

- `payload_core` だけを exact-compare してよい
- `bundle_context` と `detached_noncore` は reference-only でよい
- `must_explain` は compare target に上げない

### aggregate compare

- `summary_core` だけを exact-compare してよい
- `aggregate_context` と `detached_noncore` は reference-only でよい
- `bundle_failure_kind_counts_scope = "migrated-kinds-only"` は current aggregate core に残す

## exit code policy の最小 cut

`smoke-fixture` は「差があったかどうか」ではなく、
**loop の実行補助として成立したか**を返す helper として扱う。

したがって current judgment は次である。

- emitter failure
  - non-zero のまま返してよい
- compare helper が `0`
  - match として成功
- compare helper が `1`
  - differences found として informational に許容する
- compare helper が `2` 以上
  - helper failure として non-zero を返す

この cut にすると、新しい fixture を足した直後でも
「difference が出ること自体」は smoke を止める理由にならない。
current first tranche では、`smoke-fixture` は compare helper が `1` を返したとき
それが informational difference であることを stdout に short note として補足してよい。
current second tranche では、`compare-fixture-aggregates` を single-fixture aggregate 同士の direct compare convenience として許してよい。

## fixture authoring との関係

この helper は fixture authoring を置き換えない。

- fixture JSON と `.host-plan.json` sidecar を hand-written で整える
- `expected_static` / `expected_runtime` / `expected_trace_audit` を review する
- その後に `smoke-fixture` で detached loop へ 1 本載せる

という順番を補助するだけである。

したがって、semantics inference や expectation completion は引き続き authoring / review task 側へ残す。

## OPEN に残すもの

- `smoke-fixture` を既存 wrapper の subcommand に留めるか、別 helper に分けるか
- aggregate smoke の default compare partner を full directory 以外にも広げるか
- reference fixture compare を profile-aware selection にまで広げるか
- final path policy と final artifact retention policy
- production exporter CLI としてどこまで昇格させるか

## current judgment

- `smoke-fixture` は current detached validation loop の non-production convenience として許容してよい
- bundle compare / aggregate compare の existing helper をそのまま再利用してよい
- compare exit code `1` は differences found として informational 扱いに留めてよい
- fixture 引数は current first tranche として stem shorthand を受けてよく、missing fixture は fail-fast に止めてよい
- current second tranche では `compare-fixture-aggregates` により、single-fixture aggregate 同士の compare convenience を thin helper に入れてよい
- ただし production CLI、final exporter API、final path policy はまだ固定しない
