# examples/20 — current L2 host plan coverage failure bundle failure artifact schema

## 目的

この文書は、current L2 parser-free PoC 基盤で `host_plan_coverage_failure` を将来 bundle failure artifact 側の typed carrier に昇格させるなら、その最小 schema を docs-only でどう切るのが自然かを narrow scope で整理する。

ここで固定するのは boundary judgment だけである。production exporter 実装、actual exporter API、production host interface、runtime semantics、parser grammar は固定しない。

## 前提

- current detached artifact の success 側は bundle-first split を維持する。
  - payload core は `RunReport` 由来である。
  - `bundle_context` は `fixture_id` / `fixture_path` / `host_plan_path` / `runtime_requirement` を持つ。
  - detached non-core と human-facing explanation は success / failure どちらにもぶら下がりうるが、payload core とは分ける。
- current detached artifact では `host_plan_coverage_failure` を aggregate-only に残す judgment がある。
- 将来 typed carrier に昇格させる最小 layer は bundle failure artifact 側である。
- `fixture authoring / elaboration` は独立 bottleneck として残る。
- richer host interface は後段に残す。

## current code anchor

current code では、`host_plan_coverage_failure` は次のように materialize される。

- `RunReport` の field ではない。
- `BundleRunReport` の field でもない。
- `BatchBundleOutcome::Failed { host_plan_coverage_failure: bool }` の failure bit として現れる。
- `BatchRunSummary.host_plan_coverage_failures` は `BundleExecutionFailure` の配列として集約する。

したがって current 実装の意味としては、これは success payload に属する signal ではなく、**per-bundle failure classification が batch 集約へ持ち上がったもの** である。

## 比較する 3 schema 案

### 1. discriminator だけを持つ最小 shape

#### 形

bundle failure artifact の中に、`bundle_context` とは独立した `failure` section を置き、その中核を `failure_kind` だけにする。

```json
{
  "artifact_kind": "bundle-failure",
  "bundle_context": {
    "fixture_id": "e6-write-after-expiry",
    "fixture_path": "crates/mir-ast/tests/fixtures/current-l2/e6-write-after-expiry.json",
    "runtime_requirement": "RuntimeWithHostPlan",
    "host_plan_path": "crates/mir-ast/tests/fixtures/current-l2/e6-write-after-expiry.host-plan.json"
  },
  "failure": {
    "failure_kind": "host-plan-coverage-failure"
  }
}
```

#### 利点

- payload core を汚さない。
- success artifact に同名 field を持ち込まない。
- `bundle_context` と failure classification を混ぜない。
- batch aggregate export は `failure_kind` を集約すればよく、責務競合が起きにくい。
- richer host interface 側で typed coverage reason を導入するとき、`failure_kind` を細分化する入口として使いやすい。

#### 欠点

- triage 用の短い補助説明は別 section に逃がす必要がある。
- failure object 単体では bundle identity を持たないため、artifact 全体を見る前提になる。

#### 評価

**最小 shape として最も自然**である。

### 2. discriminator + `bundle_context` 参照を持つ shape

#### 形

`failure` section に `failure_kind` と `bundle_context_ref` を持たせる。

```json
{
  "artifact_kind": "bundle-failure",
  "bundle_context": { "...": "..." },
  "failure": {
    "failure_kind": "host-plan-coverage-failure",
    "bundle_context_ref": "#/bundle_context"
  }
}
```

#### 利点

- failure object から bundle identity へ明示的に辿れる。
- 将来 artifact を分割保存するときの参照感覚に近い。

#### 欠点

- current repo には参照規約も path policy も無い。
- 同一 artifact 内で `bundle_context` が隣にあるなら重複である。
- docs-only の narrow step に対して schema 装飾が先走る。

#### 評価

comparison 対象としては有益だが、**いま採る最小 shape ではない**。

### 3. discriminator + short failure note を持つ shape

#### 形

`failure` section に `failure_kind` と短い failure note を同居させる。

```json
{
  "artifact_kind": "bundle-failure",
  "bundle_context": { "...": "..." },
  "failure": {
    "failure_kind": "host-plan-coverage-failure",
    "short_failure_note": "host plan did not cover all oracle calls"
  }
}
```

#### 利点

- repo 外 artifact だけ見ても triage しやすい。
- current code の string error と距離が近い。

#### 欠点

- typed carrier と detached non-core explanation を混ぜやすい。
- short note がそのまま machine-check 境界へにじむ圧力を生む。
- richer host interface の typed reason 導入前に wording を固定しすぎる。

#### 評価

次点案としては有力だが、**最小 shape としては一段重い**。

## field / concept の placement

### payload core に置かないもの

- `host_plan_coverage_failure`
- `failure_kind`

理由:

- current code で `RunReport` に存在しない。
- `TraceAuditSink` の event / metadata でもない。
- current L2 の exact-compare payload core は success / evaluation result の核に留めるべきである。

### `bundle_context` に残すもの

- `fixture_id`
- `fixture_path`
- `runtime_requirement`
- optional `host_plan_path`

理由:

- これらは bundle identity / sidecar binding の情報である。
- failure classification をここへ混ぜると、成功時にも同じ箱へ failure signal を持ち込む圧力がかかる。

### `failure` section に置くもの

- `failure_kind`
  - current candidate は `host-plan-coverage-failure`

ここでは bool の `host_plan_coverage_failure` をそのまま field 名として広げるより、**typed failure discriminator** として normalise した `failure_kind` の方が自然である。

### detached non-core に置いてよいもの

- short coverage explanation
- short error summary

ただしこれは typed carrier の最小核ではない。triage 補助として optional に残す。

### human-facing explanation に残すもの

- `must_explain`
- long-form explanation
- why-this-is-good/bad の prose

これは current L2 の machine-check policy を変えない。

## `payload core` と success artifact を変えない理由

`host_plan_coverage_failure` を typed carrier に昇格させるとしても、次を変えない。

- `RunReport` payload core
- success artifact の field set

理由は次の通りである。

- uncovered oracle call は evaluation success の core result ではない。
- current code でも failure path でしか materialize されない。
- success 側へ同名 field を入れると、「failure classification が bundle context の常設 signal である」という誤読を招きやすい。

したがって narrow next step は、**failure 側だけに typed carrier を切る**ことである。

## current judgment

### current detached artifact のままなら

- aggregate-only を維持する。

### 将来 typed carrier を bundle failure artifact 側へ降ろすなら

- **first choice は discriminator-only shape**
- 次点は discriminator + short failure note
- 今回は採らない案は discriminator + `bundle_context` 参照

### first choice を採る理由

- payload core を汚さない。
- success artifact に波及しない。
- batch aggregate export は `failure_kind` の集約に戻せる。
- `bundle_context` / detached non-core / explanation の cut を維持できる。
- richer host interface typed 化の前に参照規約や prose wording を固定しすぎない。

## `fixture authoring / elaboration` との関係

`fixture authoring / elaboration` は引き続き独立 bottleneck である。

- この schema refinement は exporter 側の保存単位を整理するものであり、fixture 記述作業そのものを自動化しない。
- ただし per-bundle failure reason を typed に見返せるようになると、authoring の試行錯誤を後から横比較しやすくなる。

したがって次の切り方が自然である。

1. detached artifact の failure schema を narrow に整理する
2. fixture authoring / elaboration は独立 task として並走させる

## この task で固定しないこと

- actual exporter API
- detached artifact の保存先と path policy
- `BatchRunSummary` aggregate export がいつ typed failure carrier を吸うか
- richer host interface の typed carrier 化
- final parser syntax
- multi-request scheduler
- `Approximate` / `Compensate`

## current L2 settled / OPEN

### current L2 settled

- `host_plan_coverage_failure` は current detached artifact では aggregate-only に残す。
- 将来 typed carrier に昇格させる最小 layer は bundle failure artifact 側である。
- その next narrow step としては、`bundle_context` と独立した `failure.failure_kind` の discriminator-only shape が最も自然である。
- short coverage explanation は detached non-core に残し、typed carrier の最小核にしない。

### OPEN

- actual exporter API
- actual artifact 保存先と path policy
- `BatchRunSummary` aggregate export の閾値
- `failure_kind` を将来どこまで細分化するか
- richer host interface typed 化との接続点
