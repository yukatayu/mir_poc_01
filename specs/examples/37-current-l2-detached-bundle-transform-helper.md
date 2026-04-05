# 37 — current L2 detached bundle transform helper

## 目的

この文書は、current L2 parser-free PoC の detached validation loop で、

- `run_bundle()` が返す `BundleRunReport`
- non-production bundle-first emitter

の間に置く **repo 内 callable boundary** を、production API にしないまま最小で切り出す。

ここで固定するのは final exporter API ではない。
固定するのは、`current_l2_emit_detached_bundle.rs` の内部 private transform を、
**shared support module** へ narrow に落とす current cut だけである。

## 適用範囲

この文書は次を前提にする。

- current L2 の runtime semantics は変えない
- bundle-first detached artifact の split は `specs/examples/18-current-l2-bundle-first-detached-payload-context-split.md` に従う
- detached export loop の current docs-only judgment は `specs/examples/23-current-l2-detached-export-loop-consolidation.md` に従う
- wrapper / diff helper / path policy は current non-production boundary のまま維持する
- `host_plan_coverage_failure` は aggregate-only に残す
- public `lib.rs` / `harness.rs` API は増やさない

## current code anchor

current bundle export の call chain は次である。

```text
load_bundle_from_fixture_path()
  -> run_bundle()
  -> build_detached_bundle_artifact(...)
  -> serde_json::to_string_pretty(...)
```

このうち `build_detached_bundle_artifact(...)` は、
`crates/mir-semantics/examples/support/current_l2_detached_bundle_support.rs`
に置く repo 内 helper とする。

## なぜ helper を切り出すのか

private transform を example 本体に埋め込んだままだと、次の 3 つが分離されたまま残る。

- artifact schema の所有者が example 内 private code になる
- integration test が emitter binary 全体へ寄りすぎる
- aggregate / static-gate の shared support helper cut と非対称になる

shared support module を 1 枚入れることで、

- bundle artifact の shape を repo 内で再利用可能な callable boundary にできる
- それでも public exporter API にはしない
- Python wrapper と diff helper の narrow contract は壊さない

という cut が取れる。

## 比較した配置案

### 案 1. example 内 private transform のままにする

これは current call chain を最も動かさないが、actual narrow API cut が存在しないままになる。
したがって current step では採らない。

### 案 2. shared support module に切り出す

これを current cut とする。

```text
crates/mir-semantics/examples/support/current_l2_detached_bundle_support.rs
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
- `run_bundle()` / `BundleRunReport` の public behavior と exporter boundary を混ぜやすい
- final path policy や final compare contract まで一緒に凍らせやすい

## helper の最小責務

shared support module が持つのは次だけである。

- detached bundle artifact の carrier struct
- `FixtureBundle` と `BundleRunReport` から artifact へ写す pure transform

持たない責務は次である。

- fixture loading
- file overwrite policy
- path derivation
- compare input discovery
- diff helper
- public library/exporter API

## current minimal shape

helper の transform は次の section を作る。

1. `bundle_context`
2. `payload_core`
3. `detached_noncore`

この split 自体は既存 judgment を再利用する。

### `bundle_context`

current helper は次をそのまま構成する。

- `fixture_id`
- `fixture_path`
- `host_plan_path`
- `runtime_requirement`

### `payload_core`

current helper は次をそのまま構成する。

- `static_verdict`
- `entered_evaluation`
- `terminal_outcome`
- `event_kinds`
- formal `non_admissible_metadata`
- short `narrative_explanations`

### `detached_noncore`

current helper は次だけを current non-core anchor として残す。

- `steps_executed`

## `host_plan_coverage_failure` の扱い

current helper cut でも、`host_plan_coverage_failure` は bundle success artifact に入れない。

その理由は次の通りである。

- `RunReport` payload core の field ではない
- `FixtureBundle` identity の field でもない
- current code では batch summary 側の failure classification として materialize される

したがってこの task では、

- `bundle_context`
- `payload_core`
- `detached_noncore`

の split だけを shared helper 化し、`host_plan_coverage_failure` は引き続き aggregate 側へ残す。

## test boundary

current step では、shared support module の transform を integration test から直接呼んで確認する。

ここで machine-check に残すのは少なくとも次である。

- runtime fixture で `bundle_context` と `payload_core` が current split を保つこと
- static-only fixture で `host_plan_path = None`、`entered_evaluation = false`、`terminal_outcome = None` を保つこと
- `steps_executed` が transform source と一致すること

## current judgment

- bundle export の actual narrow API cut として、`FixtureBundle + BundleRunReport -> detached bundle artifact` の shared support module 化を採ってよい
- helper の配置は `examples/support/` が最小である
- `lib.rs` / `harness.rs` の public API はまだ増やさない
- wrapper / diff helper / path policy は current non-production boundary のまま維持する
- final exporter API、final path policy、bundle compare contract の finalization は引き続き OPEN に残す
