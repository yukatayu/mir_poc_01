# 23 — current L2 detached export loop consolidation

## 目的

この文書は、current L2 parser-free PoC 基盤で積み上げてきた detached exporter の docs-only judgment を 1 箇所へ集約し、

- 何が current L2 の current docs-only judgment として固まっているか
- 何が non-production の最小 loop attachment か
- 何が依然として OPEN か

を、次の task が無文脈でも辿れる形に整理する。

ここで fixed とするのは、runtime semantics ではなく **detached export loop の boundary** である。
production exporter API、production serialization contract、保存先 policy、richer host interface、multi-request scheduler は固定しない。

## 適用範囲

この consolidation は次を前提にする。

- current L2 の core semantics は維持する
- fallback / preference chain の reading や notation family は変えない
- detached exporter の payload core は `RunReport` 由来とする
- first exporter entry は `run_bundle` / `BundleRunReport` とする
- `fixture authoring / elaboration` は独立 bottleneck として残す
- `must_explain` は machine-check に上げない

## current docs-only judgment の集約

### 1. detached artifact の 4 層

current L2 の bundle-first detached artifact では、次の 4 層を分ける。

1. payload core
2. `bundle_context`
3. `detached_noncore`
4. human-facing explanation

これは `RunReport` payload と bundle identity / sidecar binding / operational note を混ぜないための最小 cut である。

### 2. payload core

payload core は `RunReport` と `TraceAuditSink` 由来であり、少なくとも次を持つ。

- `static_verdict`
- `entered_evaluation`
- `terminal_outcome`
- `event_kinds`
- formal `non_admissible_metadata`
- short `narrative_explanations`

current L2 では、ここが detached artifact diff の exact-compare core である。

### 3. `bundle_context`

bundle-first detached artifact では、次は payload core でも detached_noncore でもなく、独立した `bundle_context` に置く。

- `fixture_id`
- `fixture_path`
- `host_plan_path`
- `runtime_requirement`

これは `FixtureBundle` が現在すでに持っている bundle identity / sidecar binding と対応する。

### 4. `detached_noncore`

次は detached artifact に出してよいが、current L2 の exact-compare core に上げない。

- `steps_executed`
- auxiliary counters
- coverage explanation
- host-plan explanation

これらは後比較には有益だが、interpreter / helper refactor で揺れやすい。

### 5. human-facing explanation

次は detached artifact の comparison core に上げない。

- `must_explain`
- long-form audit
- why-this-is-good/bad の prose

current L2 では、これは report / docs / human-facing note の責務である。

## entry layer と helper boundary

### payload core と entry layer は分けて考える

detached exporter の current docs-only judgment は次である。

- payload core は `RunReport` に最も近い
- ただし first exporter entry は `run_bundle` / `BundleRunReport`
- `BatchRunSummary` export は後段の aggregate export

この判断を採る理由は次の通りである。

- `RunReport` だけでは bundle identity と sidecar binding を失いやすい
- `run_bundle` は `fixture + sidecar` を 1 単位として扱う current helper boundary に一致する
- `BatchRunSummary` から始めると selection / profile / aggregate concerns を一度に背負い込みやすい

## `host_plan_coverage_failure` の current state

current code では `host_plan_coverage_failure` は次の性質を持つ。

- `RunReport` の field ではない
- `BundleRunReport` の field でもない
- success-side payload core に属さない
- `batch_summary_from_discovery()` が per-bundle failure を分類して立てる
- current anchor は
  - `BatchBundleOutcome::Failed { host_plan_coverage_failure: bool }`
  - `BatchRunSummary.host_plan_coverage_failures`

したがって current detached artifact では、**aggregate-only** に残すのが current judgment である。

## future typed bundle failure artifact の最小核

`host_plan_coverage_failure` を将来 typed carrier に昇格させるなら、最小 layer は bundle failure artifact 側である。

その最小核は、`bundle_context` や short note を混ぜない **discriminator-only** である。

```json
{
  "failure": {
    "failure_kind": "host-plan-coverage-failure"
  }
}
```

ここで維持する cut は次である。

- payload core は変えない
- success artifact には同名 field を持ち込まない
- `bundle_context` は別 section のまま保つ
- short coverage explanation は detached_noncore に残す
- `must_explain` は human-facing explanation に残す

## aggregate histogram / kind count の最小候補

bundle failure artifact 側の `failure.failure_kind` を aggregate export が吸うとしても、aggregate 側は coarse summary に留める。

したがって最小 typed aggregate は、bundle failure summary の再掲ではなく **histogram / kind count** である。

current docs-only judgment では、field 名の最小候補は **`bundle_failure_kind_counts`** とする。
ただし current migration cut では migrated kind だけを数える partial histogram として始め、actual sketch では full failure histogram と誤読させない補助 field を許す。

row の最小 shape は次である。

```json
{
  "bundle_failure_kind_counts": [
    {
      "failure_kind": "host-plan-coverage-failure",
      "count": 1
    }
  ]
}
```

ここで aggregate 側へ持ち込まないものは次である。

- `bundle_context`
- detached non-core explanation
- `must_explain`
- bundle failure summary の薄い再掲

## migration cut

current docs-only の最小 migration cut は、current bool/list anchor を壊さない **additive coexistence** である。

したがって aggregate export 側では、当面次を併存させる。

- current `host_plan_coverage_failures` list
- current `BatchBundleOutcome::Failed.host_plan_coverage_failure` bool
- future `bundle_failure_kind_counts`

この cut を採る理由は次の通りである。

- current harness error wording test を直ちに揺らさない
- current batch classifier anchor を維持できる
- docs / tests / code の mirror を一度に壊さずに typed aggregate naming を先に切れる

## non-production の loop attachment

current docs-only judgment を PoC loop へつなぐ最小 operational aid として、次を許容する。

### 1. tiny bundle-first exporter / emitter

- non-production helper として置く
- `run_bundle` / `BundleRunReport` を起点にする
- payload core / `bundle_context` / `detached_noncore` を JSON 等へ落とす
- `host_plan_coverage_failure` を success payload core に入れない
- production exporter API として既成事実化しない
- actual narrow cut として、pure transform を `examples/support/current_l2_detached_bundle_support.rs` の shared support helper へ落としてよい

### 2. minimal artifact diff helper

- exact-compare core だけを比較する
- 少なくとも次を比較する
  - `static_verdict`
  - `entered_evaluation`
  - `terminal_outcome`
  - `event_kinds`
  - formal `non_admissible_metadata`
  - short `narrative_explanations`
- `detached_noncore` は参考表示に留めてよい
- `must_explain` は比較対象にしない

### 3. fixture authoring / elaboration template

`fixture authoring / elaboration` は detached exporter そのものではない。
したがって、この loop attachment は authoring bottleneck を完全解消せず、**テンプレート化 / ガイド化** に留める。

## settled current docs-only judgment と next narrow step

### settled current docs-only judgment

- payload core は `RunReport` 由来である
- first exporter entry は `run_bundle` / `BundleRunReport` である
- `bundle_context` は payload core / detached_noncore から独立させる
- `host_plan_coverage_failure` は current detached artifact では aggregate-only に残す
- future typed bundle failure artifact の最小核は `failure.failure_kind` discriminator-only である
- aggregate 側に typed 集約を持たせるなら最小は `bundle_failure_kind_counts` の histogram / kind count である
- docs-only migration cut は current bool/list anchor と typed count field の additive coexistence である

### next narrow step

current docs-only judgment の次に narrow に進めてよいのは次である。

1. non-production の tiny bundle-first exporter / emitter
2. exact-compare core に絞った minimal diff helper
3. fixture authoring / elaboration template
4. artifact 保存先 / path policy と aggregate export の最小 API cut を別文書で narrow に整理する

これらは PoC loop を回しやすくする operational aid であり、current L2 の runtime semantics や production host interface を固定しない。

## 依然として OPEN のもの

次は引き続き未決のまま残す。

- actual exporter API
- detached artifact 保存先と path policy
- aggregate export の actual implementation timing
- `bundle_failure_kind_counts` をいつ current bool/list anchor で置き換えるか
- richer host interface の typed carrier 化
- final parser syntax
- multi-request scheduler
- `Approximate` / `Compensate`
- static analysis / theorem prover 側との boundary

## source chain

この consolidation は、次の chain を統合したものである。

- `specs/examples/16-current-l2-detached-trace-audit-artifact-schema.md`
- `specs/examples/17-current-l2-detached-exporter-entry-comparison.md`
- `specs/examples/18-current-l2-bundle-first-detached-payload-context-split.md`
- `specs/examples/19-current-l2-host-plan-coverage-failure-placement.md`
- `specs/examples/20-current-l2-host-plan-coverage-failure-bundle-failure-artifact-schema.md`
- `specs/examples/21-current-l2-host-plan-coverage-failure-aggregate-connection.md`
- `specs/examples/22-current-l2-host-plan-coverage-failure-aggregate-histogram-migration.md`
- `specs/examples/24-current-l2-detached-export-storage-and-aggregate-api.md`
