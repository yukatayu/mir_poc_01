# 31 — current L2 detached aggregate transform helper

## 目的

この文書は、current L2 parser-free PoC の detached validation loop で、

- `run_directory()` が返す `BatchRunSummary`
- non-production aggregate emitter

の間に置く **repo 内 callable boundary** を、production API にしないまま最小で切り出す。

ここで固定するのは final exporter API ではない。
固定するのは、`current_l2_emit_detached_aggregate.rs` の内部 private transform を、
**shared support module** へ narrow に落とす current cut だけである。

## 適用範囲

この文書は次を前提にする。

- current L2 の runtime semantics は変えない
- detached aggregate artifact の shape は `specs/examples/25-current-l2-detached-aggregate-emitter-sketch.md` に従う
- aggregate compare helper の core / reference-only split は `specs/examples/26-current-l2-detached-aggregate-compare-helper.md` に従う
- `bundle_failure_kind_counts` と current list anchor の additive coexistence は維持する
- public `lib.rs` / `harness.rs` API は増やさない
- wrapper / compare helper / path policy は current non-production boundary のまま維持する

## current code anchor

current aggregate export の call chain は次である。

```text
run_directory()
  -> BatchRunSummary
  -> build_detached_aggregate_artifact(...)
  -> serde_json::to_string_pretty(...)
```

このうち `build_detached_aggregate_artifact(...)` は、
`crates/mir-semantics/examples/support/current_l2_detached_aggregate_support.rs`
に置く repo 内 helper とする。

## なぜ helper を切り出すのか

private transform を example 本体に埋め込んだままだと、次の 3 つが分離されたまま残る。

- artifact schema の所有者が example 内 private code になる
- compare helper が JSON field 名ベタ書きへだけ依存する
- wrapper が process boundary と path convention へだけ依存する

shared support module を 1 枚入れることで、

- aggregate artifact の shape を repo 内で再利用可能な callable boundary にできる
- それでも public exporter API にはしない
- Python wrapper と compare helper の narrow contract は壊さない

という cut が取れる。

## 比較した配置案

### 案 1. example 内 private transform のままにする

これは current call chain を最も動かさないが、actual narrow API cut が存在しないままになる。
したがって current step では採らない。

### 案 2. shared support module に切り出す

これを current cut とする。

```text
crates/mir-semantics/examples/support/current_l2_detached_aggregate_support.rs
```

この案では、

- example から `mod` で読む
- integration test から `#[path = ...]` で同じ helper を読む
- `lib.rs` では re-export しない

を同時に満たせる。

### 案 3. `src/` / `lib.rs` 側へ昇格する

これは将来の actual exporter API 候補ではあるが、current step では早い。

理由:

- public surface を既成事実化しやすい
- `run_directory()` / `BatchRunSummary` の public behavior と exporter boundary を混ぜやすい
- final path policy や final compare contract まで一緒に凍らせやすい

## helper の最小責務

shared support module が持つのは次だけである。

- detached aggregate artifact の carrier struct
- `BatchRunSummary` から artifact へ写す pure transform

持たない責務は次である。

- fixture directory 実行
- path derivation
- file overwrite policy
- compare input discovery
- aggregate compare
- public library/exporter API

## current minimal shape

helper の transform は次の section を作る。

1. `aggregate_context`
2. `summary_core`
3. `detached_noncore`

この split 自体は既存 judgment を再利用する。

### `summary_core`

current helper は次をそのまま構成する。

- `total_bundles`
- `runtime_bundles`
- `static_only_bundles`
- `passed`
- `failed`
- `bundle_failure_kind_counts_scope`
- `bundle_failure_kind_counts`

### `detached_noncore`

current helper は次を current list anchor として残す。

- `discovery_failures`
- `bundle_failures`
- `host_plan_coverage_failures`

## `host_plan_coverage_failure` の扱い

current helper cut でも、`host_plan_coverage_failure` は success-side payload core に上げない。

代わりに次の split を保つ。

- `summary_core`
  - `bundle_failure_kind_counts`
  - `bundle_failure_kind_counts_scope = "migrated-kinds-only"`
- `detached_noncore`
  - `host_plan_coverage_failures`

これにより、

- current bool/list anchor
- future typed histogram cut

の additive coexistence を helper 内 transform にまで下ろせる。

## test boundary

current step では、shared support module の transform を
integration test から直接呼んで確認する。

ここで machine-check に残すのは少なくとも次である。

- typed count row が正しく出ること
- `host_plan_coverage_failures` ref が current anchor として残ること
- coverage failure が 0 のとき typed row が空 list のままであること

## current judgment

- aggregate export の actual narrow API cut として、`BatchRunSummary -> detached aggregate artifact` の shared support module 化を採ってよい
- helper の配置は `examples/support/` が最小である
- `lib.rs` / `harness.rs` の public API はまだ増やさない
- wrapper / compare helper / path policy は current non-production boundary のまま維持する
- final exporter API、final path policy、aggregate compare contract の finalization は引き続き OPEN に残す
