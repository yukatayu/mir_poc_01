# 16 — current L2 detached trace / audit artifact schema

## 目的

この文書は、current L2 parser-free PoC の trace / audit 結果を process 内比較だけに閉じ込めず、repo 外へ保存して後で横比較できるようにするための **docs-only minimal artifact schema** を整理する。

ここで決めるのは、`RunReport` / `BundleRunReport` / `BatchRunSummary` から何を detached artifact へ出してよいか、何を exact-compare core に残すか、何を human-facing explanation に残すかの境界だけである。

この文書は production schema version、production host interface、parser grammar、runtime semantics を固定しない。

## 前提

- current L2 の core semantics は変えない
- parser-free fixture loop の machine-check surface は既存の bundle / batch helper を正本とする
- `must_explain` は current L2 でも machine-check に上げない
- richer host interface は後段 task に残す
- `fixture authoring / elaboration` は detached artifact とは別の独立 bottleneck として残る

## current carrier の位置づけ

### `ExpectedTraceAudit`

- fixture 側の expectation carrier である
- `event_kinds`
- formal `non_admissible_metadata`
- short `narrative_explanations`
- human-facing な `must_explain`

### `TraceAuditSink`

- 実行中に集める runtime sink である
- `events`
- `non_admissible_metadata`
- `narrative_explanations`
- `must_explain` は持たない

### `RunReport`

- 1 run の最小結果 carrier である
- `static_verdict`
- `entered_evaluation`
- `terminal_outcome`
- `trace_audit_sink`
- `steps_executed`

### `BundleRunReport`

- bundle helper の薄い wrapper である
- 実質的な比較責務は `RunReport` 側にある

### `BatchRunSummary`

- directory / batch 粒度の summary carrier である
- count 系
  - `total_bundles`
  - `runtime_bundles`
  - `static_only_bundles`
  - `passed`
  - `failed`
- failure 系
  - `discovery_failures`
  - `bundle_failures`
  - `host_plan_coverage_failures`
- detail 系
  - `bundle_reports`

## detached artifact へ出す field の 3 群

### 1. exact-compare core として detached artifact へ出すもの

次は current bundle / batch helper がすでに machine-check の中核として使っているため、detached artifact に出すときも exact-compare core として維持してよい。

| field | carrier | 扱い |
|---|---|---|
| `static_verdict` | `RunReport` | core |
| `entered_evaluation` | `RunReport` | core |
| `terminal_outcome` | `RunReport` | core |
| `event_kinds` | `TraceAuditSink` | core |
| formal `non_admissible_metadata` | `TraceAuditSink` | core |
| short `narrative_explanations` | `TraceAuditSink` | core |
| `host_plan_coverage_failure` | batch / per-bundle failure classification | batch-layer core |

### 1.1 `host_plan_coverage_failure` を core に入れる理由

`host_plan_coverage_failure` は current 実装では batch 層の failure classification であり、trace event 自体ではない。  
それでも detached artifact の batch-layer core に残す価値はある。

- uncovered oracle call を通常の runtime mismatch と分けて数えられる
- directory / profile 実行を後で横比較するとき、host coverage drift を素早く抽出できる

ただし current 実装は文字列検出に依存しているため、**typed carrier としては未決**である。  
この field は detached artifact へ出してよいが、production host interface の final shape を先取りしない。

### 2. detached artifact には入れてよいが machine-check core にはしないもの

次は後比較や運用上の観測には有益だが、current L2 の exact-compare core に上げると interpreter / helper refactor へ過敏になりやすい。

| field | carrier | 扱い |
|---|---|---|
| `steps_executed` | `RunReport` | detached non-core |
| coverage explanation | batch / host-side補助 | detached non-core |
| host-plan explanation | bundle / batch 補助 | detached non-core |
| auxiliary counters / summaries | `BatchRunSummary` | detached non-core |
| relative fixture path / host-plan path | bundle / batch context | detached non-core |

### 2.1 `steps_executed` を core にしない理由

`steps_executed` は実験比較や performance smoke には有益である。  
しかし current L2 では step 粒度そのものが final runtime contract ではなく、interpreter の内部 refactor で変わりうる。

そのため、detached artifact へ残すこと自体はよいが、

- semantics regression の core signal
- fixture correctness の core verdict

には使わない方が current L2 の narrow scope と整合する。

### 3. 引き続き human-facing explanation に残すもの

次は detached artifact の core / non-core にも上げず、人間向け explanation obligation として残す。

| field / 内容 | current 扱い |
|---|---|
| `must_explain` | human-facing only |
| long-form audit explanation | human-facing only |
| static verdict reason の長文 | human-facing only |
| why-this-is-good/bad の比較文 | human-facing only |

## docs-only minimal artifact schema sketch

production schema version は固定しない。  
ここでは比較補助として、current L2 で最小限必要な draft shape だけを示す。

### detached run artifact

```json
{
  "schema_version": "draft-current-l2-detached-run-v0",
  "fixture_id": "e7-write-fallback-after-expiry",
  "static_verdict": "Valid",
  "entered_evaluation": true,
  "terminal_outcome": "Success",
  "trace_audit_core": {
    "event_kinds": ["perform-success"],
    "non_admissible_metadata": [
      { "option_ref": "write-fast", "subreason": "lease-expired" }
    ],
    "narrative_explanations": [
      "capability mismatch left in narrative explanation"
    ]
  },
  "detached_noncore": {
    "steps_executed": 7
  }
}
```

### detached bundle artifact

```json
{
  "schema_version": "draft-current-l2-detached-bundle-v0",
  "runtime_requirement": "runtime-with-host-plan",
  "run_artifact": {
    "...": "detached run artifact"
  },
  "detached_noncore": {
    "fixture_path": "crates/mir-ast/tests/fixtures/current-l2/e7-write-fallback-after-expiry.json",
    "host_plan_path": "crates/mir-ast/tests/fixtures/current-l2/e7-write-fallback-after-expiry.host-plan.json"
  }
}
```

### detached batch artifact

```json
{
  "schema_version": "draft-current-l2-detached-batch-v0",
  "total_bundles": 8,
  "runtime_bundles": 6,
  "static_only_bundles": 2,
  "passed": 8,
  "failed": 0,
  "core_failures": {
    "host_plan_coverage_failures": []
  },
  "detached_noncore": {
    "bundle_reports": [],
    "auxiliary_counters": {}
  }
}
```

## machine-check と human-facing explanation の境界

### detached artifact の exact-compare core

- `static_verdict`
- `entered_evaluation`
- `terminal_outcome`
- `event_kinds`
- formal `non_admissible_metadata`
- short `narrative_explanations`
- batch-layer の `host_plan_coverage_failure`

### detached artifact に残してよいが exact-compare core にしないもの

- `steps_executed`
- coverage explanation
- host-plan explanation
- auxiliary counters / summaries

### human-facing explanation に残すもの

- `must_explain`
- long-form audit
- why-this-is-good/bad の説明

## richer host interface に後段として残す論点

次は detached artifact schema task では固定しない。

- uncovered call detection の typed 化
- preflight coverage analysis
- coverage explanation をどこへ載せるか
- `PredicateOracle` / `EffectOracle` carrier をどこまで増やすか

これらは richer host interface の後段 task に残す。

## current L2 でまだ決めないこと

- production serialization format
- schema versioning rule の固定
- detached artifact の保存パス規約
- typed `host_plan_coverage_failure` carrier
- richer host interface の typed carrier
- multi-request scheduler との接続
- static analysis / theorem prover 側との boundary

## provisional judgment

current L2 の next narrow step としては、

1. detached trace / audit の docs-only minimal artifact schema を切る
2. exact-compare core と detached non-core を分ける
3. `must_explain` は human-facing explanation に残す
4. richer host interface は後段に回す

の順が最小である。
