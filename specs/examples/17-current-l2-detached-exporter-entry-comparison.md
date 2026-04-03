# 17 — current L2 detached exporter entry comparison

## 目的

この文書は、current L2 parser-free PoC で detached trace / audit artifact exporter を narrow に始めるなら、どの layer を entry にするのが最小で比較価値が高いかを整理する。

production exporter 実装、production schema version、richer host interface はここでは固定しない。
ここで行うのは **entry layer comparison** のみである。

## 前提

- current L2 の core semantics は維持する。
- fallback / preference chain の semantics や notation family は変更しない。
- detached trace / audit artifact の docs-only minimal schema は `specs/examples/16-current-l2-detached-trace-audit-artifact-schema.md` を正本とする。
- `fixture authoring / elaboration` は独立 bottleneck として残る。
- richer host interface は後段 comparison に残す。

## current carrier / helper の位置

current L2 parser-free PoC では、次の carrier と helper が exporter entry 比較の主対象になる。

- `RunReport`
  - interpreter / host 実行 1 回分の最小結果 carrier。
  - `static_verdict`、`entered_evaluation`、`terminal_outcome`、`TraceAuditSink`、`steps_executed` を持つ。
- `BundleRunReport`
  - 1 bundle 実行の wrapper。
  - `report: RunReport` を持つ。
  - 実際の entry helper は `run_bundle` であり、fixture path、host-plan path、runtime/static classification、coverage failure と隣接している。
- `BatchRunSummary`
  - bundle 群の集計 carrier。
  - total / passed / failed / coverage failure と `bundle_reports` を持つ。

## 比較候補

### Candidate 1. `RunReport` 直列 export

#### 利点

- exact-compare core に最も近い。
- `TraceAuditSink` と 1 対 1 に近い payload を切り出しやすい。
- detached artifact schema の `static_verdict`、`entered_evaluation`、`terminal_outcome`、`event_kinds`、formal `non_admissible_metadata`、short `narrative_explanations` と直接対応づけやすい。

#### 欠点

- fixture path、host-plan path、runtime requirement、bundle 単位の operational context を自前で後付けしないと artifact 比較に使いにくい。
- exporter の entry を interpreter / host harness の低層に近づけすぎる。
- helper boundary の観点では、bundle 層が持っている verification context を捨ててしまう。

#### 判断

`RunReport` は **payload core** としては最有力だが、最初の exporter entry boundary としては低すぎる。

### Candidate 2. `BundleRunReport` export

#### 利点

- payload core は `RunReport` をそのまま再利用できる。
- exporter entry を `run_bundle` に置けるので、current helper boundary を壊しにくい。
- fixture path、host-plan path、runtime/static classification、coverage failure と detached non-core context を 1 artifact にまとめやすい。
- `fixture authoring / elaboration` bottleneck とも噛み合う。
  - bundle は「fixture + sidecar」の単位なので、authoring 側も比較しやすい。

#### 欠点

- `BundleRunReport` 自体は thin wrapper なので、artifact shape を考えると `RunReport` payload と bundle context の二層整理が必要になる。
- coverage failure は trace event ではないため、artifact 内でも core / non-core の分離を明示しないと混ざりやすい。

#### 判断

最初の exporter entry layer としては最も自然である。
payload core は `RunReport` 由来のまま保ち、entry boundary だけを bundle 層に置く構図が current helper stack と最も整合する。

### Candidate 3. `BatchRunSummary` export

#### 利点

- 大量比較や profile 実行の横比較には便利である。
- coverage failure、passed / failed、bundle count をまとめて扱える。

#### 欠点

- 最初の exporter entry としては coarse すぎる。
- discovery / selection / profile / named profile の集計文脈をまとめて背負い込むため、detached artifact exporter の責務が急に広がる。
- current `host_plan_coverage_failure` が batch summary では集計的に表れているため、typed 化していない operational detail まで一度に背負い込みやすい。

#### 判断

第 1 段階ではなく、第 2 段階の aggregate export として扱うべきである。

## 比較表

| entry 候補 | exact-compare core との近さ | detached non-core の扱いやすさ | helper boundary への波及 | repo 外再比較 | 初手としての評価 |
|---|---|---|---|---|---|
| `RunReport` | 最も近い | 弱い | interpreter / host 側へ寄りすぎる | payload 単体では弱い | payload core として採用、entry は見送る |
| `BundleRunReport` | 十分近い | 最も扱いやすい | `run_bundle` 境界に留めやすい | fixture / sidecar 単位で比較しやすい | **最初の entry として採用** |
| `BatchRunSummary` | 間接的 | 集計には強い | batch / selection / profile へ広がる | profile 比較には強い | 第 2 段階へ回す |

## current judgment

### payload core

detached artifact の payload core は `RunReport` 由来とする。
ここでの core とは、`specs/examples/16-current-l2-detached-trace-audit-artifact-schema.md` で exact-compare core とされた部分である。

### first exporter entry

最初に切るべき exporter entry boundary は `run_bundle` / `BundleRunReport` である。

理由は次の通りである。

- current helper stack の public behavior を壊しにくい。
- bundle は fixture authoring / elaboration の自然単位である。
- payload core と detached non-core context を無理なく分けられる。
- `BatchRunSummary` より narrow で、`RunReport` 単体より operational comparison に使いやすい。

### second stage

`BatchRunSummary` export は、第 1 段階の bundle-level exporter が安定した後に検討する。
最初から batch 層で始めると、profile / selection / coverage aggregation の論点まで一度に固定圧がかかる。

## この task で固定しないこと

次は引き続き未決とする。

- production exporter 実装
- production schema version
- richer host interface の typed carrier
- coverage failure carrier の最終 typed 化
- repo 外保存パス規約
- multi-request scheduler との接続
- static analysis / theorem prover 側との連携方法

## current L2 settled / OPEN

### current L2 settled

- detached artifact の payload core は `RunReport` に最も近い
- ただし first exporter entry boundary は `run_bundle` / `BundleRunReport` が最小である
- `BatchRunSummary` は第 2 段階の aggregate export に回す
- `fixture authoring / elaboration` は independent bottleneck として残る

### OPEN

- actual exporter API の形
- detached artifact 保存先と path policy
- batch / profile / named profile をまとめた aggregate export
- richer host interface との typed 接続
