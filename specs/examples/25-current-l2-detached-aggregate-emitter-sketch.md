# 25 — current L2 detached aggregate emitter sketch

## 目的

この文書は、current L2 parser-free PoC の detached validation loop で、

- bundle-first artifact に加えて
- batch aggregate summary も
- non-production helper として narrow に吐く

ための最小 cut を整理する。

ここで固定するのは production exporter API ではない。
固定するのは、`run_directory` / `BatchRunSummary` を起点にした **aggregate detached artifact の non-production sketch** だけである。

## 適用範囲

この文書は次を前提にする。

- current L2 の core semantics は変えない
- bundle-first detached artifact の payload/context split は維持する
- `host_plan_coverage_failure` は success-side payload core へ逆流させない
- future typed aggregate の最小候補は `bundle_failure_kind_counts`
- current bool/list anchor と typed count field の additive coexistence を維持する
- richer host interface、profile-aware aggregate export、final path policy は固定しない

## current non-production aggregate emitter の位置づけ

current detached validation loop の aggregate 側 actual narrow cut は、
`crates/mir-semantics/examples/current_l2_emit_detached_aggregate.rs` を non-production helper として置くところまでである。

この helper は次を行う。

- 入力として fixture directory を受け取る
- `run_directory()` を起点に `BatchRunSummary` を得る
- coarse summary を detached aggregate artifact として JSON に落とす

この helper は次を行わない。

- `harness.rs` / `lib.rs` の public API を増やすこと
- profile / alias / scheduler の semantics を固定すること
- bundle failure summary の薄い再掲を aggregate core に持ち込むこと
- final serialization contract を定めること

## aggregate artifact の最小 shape

current non-production aggregate artifact は、少なくとも次の 3 section を持つ。

1. `aggregate_context`
2. `summary_core`
3. `detached_noncore`

### `aggregate_context`

最小では次を持つ。

- `directory_path`
- `aggregate_scope`

current sketch では `aggregate_scope = "directory-all"` とし、directory 全体をそのまま集計したことだけを示す。

### `summary_core`

aggregate 側の minimal core は coarse summary に留め、次を持つ。

- `total_bundles`
- `runtime_bundles`
- `static_only_bundles`
- `passed`
- `failed`
- `bundle_failure_kind_counts_scope`
- `bundle_failure_kind_counts`

ここで `bundle_failure_kind_counts` は typed aggregate の最小候補であり、**migrated kind だけを数える partial histogram** として row list で持つ。
current actual sketch では、その partial 性を `bundle_failure_kind_counts_scope = "migrated-kinds-only"` で明示する。

```json
{
  "bundle_failure_kind_counts_scope": "migrated-kinds-only",
  "bundle_failure_kind_counts": [
    {
      "failure_kind": "host-plan-coverage-failure",
      "count": 1
    }
  ]
}
```

current non-production sketch では、count が 0 のとき row を増やさず空 list にしてよい。

### `detached_noncore`

aggregate 側の list anchor と explanation 寄り carrier は core に混ぜず、`detached_noncore` に残す。

- `discovery_failures`
- `bundle_failures`
- `host_plan_coverage_failures`

この cut により、

- coarse summary と typed count field は `summary_core`
- 現在の list anchor と explanatory carrier は `detached_noncore`

に分かれる。

## `host_plan_coverage_failure` の位置

current aggregate emitter sketch では、`host_plan_coverage_failure` を次のように扱う。

- `summary_core`
  - `bundle_failure_kind_counts` に typed count として反映してよい
  - ただし current sketch では migrated kind だけを数える partial histogram として扱う
- `detached_noncore`
  - current `host_plan_coverage_failures` list を残してよい
- 持ち込まない場所
  - success-side payload core
  - `bundle_context`
  - bundle failure summary の薄い再掲

つまり current non-production sketch でも、**typed count field と current list anchor の additive coexistence** を維持する。

ただし code anchor にある `BatchBundleOutcome::Failed.host_plan_coverage_failure` bool を aggregate artifact にそのまま再掲する必要はない。
その bool は current classifier anchor として code に残し、aggregate artifact 側では list anchor と typed count field の coexistence を見せれば十分とする。

## storage / path policy との接続

current non-production default candidate は `specs/examples/24-current-l2-detached-export-storage-and-aggregate-api.md` と同じく
`target/current-l2-detached/` とする。

aggregate artifact の current naming は次を最小候補とする。

```text
target/current-l2-detached/
  aggregates/
    <run-label>/
      batch-summary.detached.json
```

この naming は current detached validation loop の convenience に限る。
final path canonicalization policy ではない。

## tiny loop wrapper との接続

current detached validation loop の tiny wrapper は、aggregate 側では少なくとも次を支えてよい。

- `emit-aggregate`
  - fixture directory から aggregate detached artifact を吐く
- `--artifact-root`
  - current non-production storage root を override できる
- `--run-label`
  - aggregate artifact を人間が run 単位に区別できる
- `--overwrite`
  - fail-closed default を明示的 override するときだけ上書きする

wrapper は production CLI ではない。
aggregate compare を汎用化しないまま、まず artifact 保存と smoke evidence の入口だけを与える。

## 何をまだ OPEN に残すか

次は引き続き未決のまま残す。

- actual exporter API を `lib.rs` / `harness.rs` にどう切るか
- aggregate emitter を named profile / profiled summary へ広げる timing
- `bundle_failure_kind_counts` を row list のままにするか object map にするか
- current list anchor をいつ除去するか
- final path canonicalization policy
- aggregate compare helper をどこまで一般化するか
- richer host interface の typed coverage carrier とどう接続するか

## current judgment

current non-production aggregate emitter sketch の judgment は次である。

- aggregate export は `run_directory` / `BatchRunSummary` 起点で narrow に始めてよい
- aggregate artifact は `aggregate_context` / `summary_core` / `detached_noncore` に分ける
- typed aggregate の最小 field は `bundle_failure_kind_counts`
- current actual sketch では `bundle_failure_kind_counts_scope = "migrated-kinds-only"` を併せて持ち、full failure histogram と誤読させない
- current `host_plan_coverage_failures` list は `detached_noncore` に残し、typed count field と additive coexistence させてよい
- bundle failure summary の薄い再掲は current L2 では採らない
- production API と final path policy はまだ固定しない
