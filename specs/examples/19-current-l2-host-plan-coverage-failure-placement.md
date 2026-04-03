# examples/19 — current L2 host plan coverage failure placement

## 目的

この文書は、current L2 parser-free PoC 基盤で `host_plan_coverage_failure` を将来 typed carrier に切り出すとしたら、どの layer に置くのが最も自然かを narrow scope で比較する。

ここで固定するのは docs-only の boundary judgment だけである。production exporter 実装、production host interface、runtime semantics、parser grammar は固定しない。

## 前提

- current detached artifact の payload core は `RunReport` 由来である。
- first exporter entry は `run_bundle` / `BundleRunReport` である。
- bundle-first detached artifact では `host_plan_coverage_failure` を aggregate-only に残す current judgment がある。
- `fixture authoring / elaboration` は独立 bottleneck として残る。
- richer host interface は後段に残す。

## current code での実体

current code では、`host_plan_coverage_failure` は次の性質を持つ。

- `RunReport` の field ではない。
- `BundleRunReport` の field でもない。
- `TraceAuditSink` の event / metadata でもない。
- `batch_summary_from_discovery()` が bundle 実行 failure の error text から分類して立てる。
- materialize 先は
  - `BatchBundleOutcome::Failed { host_plan_coverage_failure: bool }`
  - `BatchRunSummary.host_plan_coverage_failures`
  である。

したがって current 実装の意味としては、これは run payload ではなく **bundle failure classification が batch 集約へ持ち上がったもの** である。

## 比較する 3 案

### 1. aggregate-only を維持する

#### 利点

- current docs-only split と一致する。
- bundle-first payload/context split を汚さない。
- `RunReport` payload core と `bundle_context` の境界を保ちやすい。
- typed carrier を急いで固定しなくてよい。

#### 欠点

- per-bundle failure の比較をしたいときに coarse である。
- future exporter で batch summary だけを見ないと coverage failure が読めない。
- current code が per-bundle failure bit をすでに持っているのに、docs 上では aggregate に押し込めたままになる。

#### 評価

current detached artifact の最小形としては妥当である。  
ただし **typed carrier を将来どこに切るべきか** という問いへの答えとしては coarse に留まる。

### 2. bundle failure artifact 側に typed carrier を導入する

#### 利点

- current code の実体に最も近い。
  - `BatchBundleOutcome::Failed { host_plan_coverage_failure: bool }`
  - `BatchBundleReport`
- payload core を汚さずに per-bundle failure identity を持てる。
- batch aggregate export と競合しない。
  - batch 層は bundle failure artifact の集約に戻れる。
- future host interface の typed 化とも接続しやすい。
  - uncovered call detection が string 判定から typed reason へ移るとき、failure artifact の reason field に昇格しやすい。

#### 欠点

- current repo にはまだ独立した bundle failure artifact schema が無い。
- bundle-first success artifact と failure artifact をどう並べるかは別 task が要る。

#### 最小 shape の例

もし narrow に切るなら、最低限必要なのは次で十分である。

- `fixture_id`
- `fixture_path`
- `runtime_requirement`
- `failure_kind`
  - current L2 では `host-plan-coverage-failure`
- optional `host_plan_path`
- optional short `error_summary`

この shape なら、success payload core / bundle_context / detached non-core の split を壊さず、failure 側だけに typed carrier を置ける。

#### 評価

**将来 typed carrier に昇格させる最小 layer として最も自然**である。

### 3. detached non-core / `bundle_context` に薄く置く

#### 利点

- bundle-first artifact だけ見ても coverage failure の有無を読みやすい。
- schema 追加が少なく見える。

#### 欠点

- success artifact にも同じ field を持たせる圧がかかりやすい。
- `bundle_context` は identity / sidecar binding の層なので、failure classification を混ぜると責務が濁る。
- detached non-core は auxiliary / explanation 寄りの箱なので、operational failure reason を薄めすぎる。
- current code の実体とも一致しない。

#### 評価

最小に見えて、payload/context/explanation の cut を崩しやすい。  
current L2 では採らない。

## `payload core` に上げない理由

`host_plan_coverage_failure` を payload core に上げない理由は明確である。

- `RunReport` に存在しない。
- `TraceAuditSink` に存在しない。
- current semantics の trace / audit event ではない。
- uncovered oracle call の operational failure classification であって、run result core ではない。

したがって current L2 では、payload core に上げるのは不自然である。

## current judgment

### current detached artifact のままなら

- **aggregate-only を維持する。**

これは bundle-first payload/context split を保つ current judgment と整合する。

### 将来 typed carrier に昇格させるなら

- **bundle failure artifact 側に切るのが最も自然である。**

理由は次の通りである。

- current code がすでに per-bundle failure classification を持っている。
- payload core を汚さない。
- `bundle_context` と detached non-core の責務を崩さない。
- `BatchRunSummary` はその typed bundle failure を集約する後段に留められる。
- richer host interface typed 化へも接続しやすい。

### 今回採らない案

- detached non-core / `bundle_context` へ薄く置く案
- payload core へ上げる案

## `fixture authoring / elaboration` との関係

`fixture authoring / elaboration` は引き続き独立 bottleneck である。

- bundle failure artifact を typed 化しても、fixture 追加や sidecar 記述が自動で楽になるわけではない。
- ただし per-bundle failure を repo 外 artifact で後比較しやすくなるため、authoring の試行錯誤を後から見返す助けにはなる。

したがって、先後関係としては

1. detached boundary を narrow に整える
2. fixture authoring / elaboration 改善は独立に並走させる

の方が自然である。

## この task で固定しないこと

- actual exporter API
- bundle failure artifact の final serialization
- detached artifact 保存先と path policy
- richer host interface の typed carrier 化
- uncovered call detection をどこで typed にするかの最終判断
- final parser syntax
- multi-request scheduler
- `Approximate` / `Compensate`

## current L2 settled / OPEN

### current L2 settled

- `host_plan_coverage_failure` は current detached artifact では aggregate-only に残す。
- payload core に上げない。
- detached non-core / `bundle_context` に薄く混ぜない。
- 将来 typed carrier に昇格させる最小 layer は bundle failure artifact 側である。

### OPEN

- bundle failure artifact の実際の schema
- aggregate export がその typed carrier をいつ吸うか
- richer host interface typed 化との接続点
- string detection をどの段階で typed reason に置き換えるか
