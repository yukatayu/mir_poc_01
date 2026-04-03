# 18 — current L2 bundle-first detached payload/context split

## 目的

この文書は、current L2 parser-free PoC で detached artifact exporter を `run_bundle` / `BundleRunReport` から narrow に始めるとき、

- 何を payload core に置くか
- 何を bundle-level context に置くか
- 何を detached non-core に置くか
- 何を human-facing explanation に残すか

を docs-only で整理する。

production exporter 実装、production schema version、richer host interface、runtime semantics はここでは固定しない。

## 前提

- detached artifact の payload core は `RunReport` に最も近い。
- first exporter entry boundary は `run_bundle` / `BundleRunReport` である。
- `BatchRunSummary` export は後段の aggregate export に回す。
- `fixture authoring / elaboration` は independent bottleneck として残る。
- `must_explain` は machine-check に上げない。

## current carrier / helper の位置

### `RunReport`

`RunReport` は run payload core であり、次だけを持つ。

- `static_verdict`
- `entered_evaluation`
- `terminal_outcome`
- `trace_audit_sink`
- `steps_executed`

ここには bundle identity や host-plan path は入っていない。

### `FixtureBundle`

bundle helper が実際に持っている bundle-level context は `FixtureBundle` 側にある。

- `fixture_path`
- `host_plan_path`
- `fixture`
  - ここから `fixture_id` を読める
- `runtime_requirement`

### `BundleRunReport`

`BundleRunReport` は `report: RunReport` だけを持つ thin wrapper である。
したがって bundle-first exporter を考えるときは、**`BundleRunReport` 単体ではなく `run_bundle` が受け取る `FixtureBundle` と、返す `BundleRunReport` の組**を見る必要がある。

## 3 群の cut

### 1. exact-compare payload core

bundle-first exporter でも、payload core は `RunReport` 由来のまま保つ。

- `static_verdict`
- `entered_evaluation`
- `terminal_outcome`
- `event_kinds`
- formal `non_admissible_metadata`
- short `narrative_explanations`

ここでの `event_kinds` / formal `non_admissible_metadata` / short `narrative_explanations` は `trace_audit_sink` から導く。

### 2. bundle-level context

bundle-first exporter では、次は detached non-core の雑多な残り物ではなく、**独立した `bundle_context`** に置くのが最も自然である。

- `fixture_id`
- `fixture_path`
- `host_plan_path`
- `runtime_requirement`

これらは current helper stack で 1 bundle の identity と sidecar binding を表す比較的安定した座標であり、

- pure payload core
- auxiliary non-core

のどちらにも素直には入らない。

### 3. detached non-core

bundle-first exporter でも、次は detached artifact に入れてよいが core にしない。

- `steps_executed`
- auxiliary counters
- coverage explanation
- host-plan explanation

これらは後比較や後解析には便利だが、interpreter / helper refactor で揺れやすい。

### 4. human-facing explanation

次は detached artifact の machine-check surface に上げない。

- `must_explain`
- long-form audit
- why-this-is-good/bad の prose
- report に書く比較理由

## `fixture_id` / `fixture_path` / `host_plan_path` / `runtime_requirement` の placement 比較

### Candidate A. top-level field に並べる

#### 利点

- 目で追いやすい
- JSON viewer で開いたときの可読性は高い

#### 欠点

- payload core と bundle context の cut が弱く見える
- `RunReport` 由来の core と同じ層に見え、bundle identity と run payload を混同しやすい

### Candidate B. `detached_noncore` にまとめる

#### 利点

- payload core と分離できる
- schema は単純になる

#### 欠点

- `steps_executed` のような auxiliary field と、`fixture_path` / `runtime_requirement` のような bundle identity field が同じ箱に入ってしまう
- context と auxiliary の意味差が見えにくい

### Candidate C. `bundle_context` として独立 section に切る

#### 利点

- payload core と bundle identity の cut が最も明瞭である
- `FixtureBundle` が現在すでに持っている座標と対応づけやすい
- detached non-core を auxiliary / explanation 寄りの field に保ちやすい

#### 欠点

- section が 1 つ増える

### judgment

current L2 の bundle-first exporter では、**Candidate C の `bundle_context` 独立 section** が最も自然である。

## `host_plan_coverage_failure` の placement 比較

### payload core に置く案

#### 置かない理由

- `RunReport` / `TraceAuditSink` の field ではない
- trace event でもない
- `run_bundle` の success payload から直接は得られない

したがって payload core には置かない。

### detached non-core に置く案

#### 利点

- 「bundle 実行の補足情報」としては見える

#### 置かない理由

- current code では bundle context から自然に出る field ではなく、`batch_summary_from_discovery` の中で failure text から分類される operational result である
- 成功 bundle artifact にも常に持たせると、存在形が不自然になる
- bundle-first exporter に無い typed failure carrier を先取りしやすい

したがって current L2 の minimal split では detached non-core にも置かない。

### aggregate-only に置く案

#### 置く理由

- current code での実体が batch / per-bundle failure classification だからである
- `specs/examples/16-current-l2-detached-trace-audit-artifact-schema.md` でも batch-layer core として扱っている
- bundle-first exporter を narrow に始める段階で、typed coverage carrier を先取りせずに済む

#### 欠点

- bundle 単位 artifact だけを見ると coverage failure が直には見えない

### judgment

current L2 の bundle-first detached artifact では、**`host_plan_coverage_failure` は aggregate-only に残す**のが最も自然である。

将来、bundle-level failure artifact や typed coverage carrier を導入する場合は、別 task で再比較する。

## docs-only minimal schema sketch

production schema version は固定しない。
ここでは comparison aid として最小 shape だけを示す。

```json
{
  "schema_version": "draft-current-l2-detached-bundle-v0",
  "bundle_context": {
    "fixture_id": "e7-write-fallback-after-expiry",
    "fixture_path": "crates/mir-ast/tests/fixtures/current-l2/e7-write-fallback-after-expiry.json",
    "host_plan_path": "crates/mir-ast/tests/fixtures/current-l2/e7-write-fallback-after-expiry.host-plan.json",
    "runtime_requirement": "runtime-with-host-plan"
  },
  "payload_core": {
    "static_verdict": "valid",
    "entered_evaluation": true,
    "terminal_outcome": "success",
    "trace_audit_core": {
      "event_kinds": ["perform-success"],
      "non_admissible_metadata": [
        { "option_ref": "write-fast", "subreason": "lease-expired" }
      ],
      "narrative_explanations": [
        "capability mismatch left in narrative explanation"
      ]
    }
  },
  "detached_noncore": {
    "steps_executed": 7
  }
}
```

### この sketch に含めないもの

- `host_plan_coverage_failure`
  - batch / aggregate 側に残す
- `must_explain`
  - human-facing explanation に残す
- long-form audit
  - human-facing explanation に残す

## `RunReport` payload core との関係

bundle-first exporter は、新しい payload を invent しない。
やることは次の 2 段だけである。

1. `RunReport` 由来の exact-compare core を `payload_core` として保持する
2. `FixtureBundle` 由来の bundle identity / sidecar binding を `bundle_context` として外付けする

この二層化により、payload と context の責務を混ぜずに exporter を narrow に始められる。

## この task で固定しないこと

- actual exporter API
- detached artifact 保存先と path policy
- bundle failure artifact の final shape
- `host_plan_coverage_failure` の typed bundle-level carrier
- `BatchRunSummary` aggregate export の閾値
- richer host interface の typed carrier 化
- final parser syntax
- multi-request scheduler
- `Approximate` / `Compensate`

## current L2 settled / OPEN

### current L2 settled

- bundle-first exporter の payload core は `RunReport` 由来である
- `fixture_id` / `fixture_path` / `host_plan_path` / `runtime_requirement` は `bundle_context` として独立 section に置く
- `steps_executed` は detached non-core に置く
- `host_plan_coverage_failure` は bundle-first artifact には入れず、aggregate-only に残す
- `must_explain` は human-facing explanation に残す

### OPEN

- actual exporter API
- detached artifact 保存パス規約
- bundle-level failure artifact の shape
- aggregate export との接続
- typed coverage carrier
