# examples/22 — current L2 host plan coverage failure aggregate histogram migration

## 目的

この文書は、current L2 parser-free PoC 基盤で bundle failure artifact 側の `failure.failure_kind` を aggregate export が histogram / kind count として吸うなら、その **field 名、shape、migration cut** を docs-only で narrow に比較する。

ここで固定するのは docs-only boundary refinement だけである。production exporter 実装、actual exporter API、runtime semantics、parser grammar、failure family は固定しない。

## 前提

- current detached exporter chain では、次が source-backed judgment である。
  - `host_plan_coverage_failure` は current detached artifact では aggregate-only に残す。
  - 将来 typed carrier に昇格させる最小核は bundle failure artifact 側の `failure.failure_kind` discriminator-only である。
  - aggregate export が typed bundle failure を吸うとしても、持たせる typed 集約は histogram / kind count までに留める。
  - bundle failure summary の薄い再掲は採らない。
- `fixture authoring / elaboration` は独立 bottleneck として残る。
- richer host interface は後段に残す。

## current code anchor

current code では、`host_plan_coverage_failure` は次のように materialize される。

- `RunReport` の field ではない。
- `BundleRunReport` の field でもない。
- `BatchBundleOutcome::Failed { host_plan_coverage_failure: bool }` の failure bit として現れる。
- `BatchRunSummary.host_plan_coverage_failures` は `BundleExecutionFailure` の配列として集約する。
- `batch_summary_from_discovery()` が `error_text.contains("host plan did not cover all oracle calls")` でこれを分類する。
- tests では summary / batch report だけでなく、harness layer の raw error wording も固定している。

したがって migration cut は aggregate field 名だけの問題ではなく、次の anchor にまたがる。

1. host stub の raw error wording
2. batch classifier
3. `BatchBundleOutcome::Failed.host_plan_coverage_failure`
4. `BatchRunSummary.host_plan_coverage_failures`
5. それらを前提にした tests / docs / plan mirror

## 比較する aggregate shape 案

### 1. current bool/list counter 維持

aggregate は current code と同様に、`host_plan_coverage_failures` list と `BatchBundleOutcome::Failed.host_plan_coverage_failure` bool だけを持つ。

#### 利点

- current helper stack と完全に一致する。
- docs / tests / code の current anchor を増やさない。

#### 欠点

- bundle failure artifact 側の `failure.failure_kind` と aggregate 側の naming がつながらない。
- typed histogram / kind count という next narrow step の比較には情報が足りない。

### 2. typed histogram / kind count に置き換える

aggregate 側の `host_plan_coverage_failures` list をやめ、typed count field だけを持つ。

#### 利点

- aggregate naming は一気に typed になる。
- future bundle failure artifact との接続は単純になる。

#### 欠点

- current code / tests / report chain が依存する bool/list anchor を一度に崩しやすい。
- `BatchBundleOutcome::Failed.host_plan_coverage_failure` bool と aggregate field の migration を同時に決める圧力が生まれる。
- docs-only task としては cut が大きすぎる。

### 3. bool/list と typed histogram を併存させる

current bool/list anchor を残したまま、aggregate 側へ typed count field を additive に足す。

#### 利点

- current code anchor を壊さずに typed aggregate naming を先に切れる。
- batch summary の coarse role を維持したまま、bundle failure artifact 側の `failure.failure_kind` と接続できる。
- replacement timing を actual exporter API task まで deferred にできる。

#### 欠点

- 一時的に aggregate 内の failure 集約が二重化する。
- docs / tests / plan mirror で migration 状態を明示しないと drift source になる。

#### 評価

docs-only の **最小 migration cut** としては、この併存案が最も自然である。

## field 名候補の比較

### 候補 A. `typed_failure_histogram`

```json
{
  "typed_failure_histogram": [
    {
      "failure_kind": "host-plan-coverage-failure",
      "count": 1
    }
  ]
}
```

#### 利点

- typed aggregate であることは分かる。
- `histogram` という語で count 集約の意図を伝えやすい。

#### 欠点

- `typed_failure` が何に対して typed なのかが曖昧である。
- current helper stack にある `bundle_failures` / `discovery_failures` の naming family とつながりにくい。
- future discovery-failure kind まで同じ field が吸うような誤読を誘いやすい。

### 候補 B. `failure_kind_histogram`

```json
{
  "failure_kind_histogram": [
    {
      "failure_kind": "host-plan-coverage-failure",
      "count": 1
    }
  ]
}
```

#### 利点

- bundle failure artifact 側の `failure.failure_kind` と直接つながる。
- `histogram` により count 集約だと分かる。

#### 欠点

- aggregate が bundle failure と discovery failure を分けている現状を field 名だけでは表せない。
- summary field としてはやや抽象的で、current `bundle_failures` family より一段ずれて見える。

### 候補 C. `bundle_failure_kind_counts`

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

#### 利点

- `bundle_` prefix が current `BatchRunSummary` の `bundle_failures` / `bundle_reports` と整合する。
- `failure_kind` が bundle failure artifact 側の `failure.failure_kind` と 1 対 1 でつながる。
- `counts` が coarse summary role を素直に表し、bundle failure summary 再掲を誘いにくい。
- discovery 側の future typed aggregate が後で必要になっても、`discovery_failure_kind_counts` のように並べやすい。

#### 欠点

- `histogram` という語に比べると視覚的な compactness は少し落ちる。
- row list か map/object かは field 名だけでは決まらない。

#### 評価

**field 名の最小候補としては `bundle_failure_kind_counts` が最も自然**である。

## shape の比較

### row list

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

#### 利点

- `failure_kind` と `count` の pair を明示できる。
- docs-only 段階で key canonicalization や object member ordering を固定せずに済む。
- future に known kind が増えても、schema の見え方を変えずに追記できる。

#### 欠点

- object map よりは冗長である。

### object map

```json
{
  "bundle_failure_kind_counts": {
    "host-plan-coverage-failure": 1
  }
}
```

#### 利点

- compact である。
- consumer 側で count lookup はしやすい。

#### 欠点

- docs-only の段階で key canonicalization と member ordering の期待を招きやすい。
- future failure kind の文字列 policy を object key として早く固定する圧力がある。

#### 評価

current L2 docs-only 段階では、**row list の方が最小**である。object map は actual exporter API と serialization contract を切る task まで OPEN に残す。

## migration cut の比較

### 置換

- `host_plan_coverage_failures` list を `bundle_failure_kind_counts` に置き換える
- 必要なら `BatchBundleOutcome::Failed.host_plan_coverage_failure` bool も後追いで落とす

#### 評価

docs-only の next narrow step としては大きすぎる。current test / report / mirror anchor を一度に揺らしやすいため、今回は採らない。

### 併存

- `host_plan_coverage_failures` list を残す
- `BatchBundleOutcome::Failed.host_plan_coverage_failure` bool も残す
- aggregate 側へ `bundle_failure_kind_counts` を additive に足す

#### 評価

**current repo の next narrow step として最も自然**である。typed aggregate の naming と shape を先に切りつつ、current helper stack の raw error wording / classifier / summary test anchor を壊さない。

### まだ保留

- field 名だけ決め、併存か置換かは何も言わない

#### 評価

comparison はできるが、migration cut が未整理のまま残り、docs / plan / report mirror で drift を呼びやすい。今回の narrow refinement としては不十分である。

## current judgment

### aggregate export 側の最小 typed 集約 shape

- field 名の最小候補は **`bundle_failure_kind_counts`**
- shape は **row list**
- row は
  - `failure_kind`
  - `count`
  だけを持つ

### migration cut

- docs-only の next narrow step は **bool/list と typed count field の併存**
- したがって current `host_plan_coverage_failures` list はただちに置換しない
- `BatchBundleOutcome::Failed.host_plan_coverage_failure` bool もこの task では触らない
- 置換 timing は actual exporter API と implementation task まで OPEN に残す

### bundle artifact と aggregate の責務

- bundle failure artifact に残すもの
  - `failure.failure_kind`
  - `bundle_context`
- aggregate に出すもの
  - `bundle_failure_kind_counts`
- aggregate に出さないもの
  - bundle failure summary の再掲
  - detached non-core explanation
  - `must_explain`

## `fixture authoring / elaboration` との関係

`fixture authoring / elaboration` はこの task と独立 bottleneck である。

- histogram naming と migration cut を narrow に切っても、fixture 自体を書く cost は減らない
- ただし typed count field があれば、多件実行時に coarse aggregate を見比べやすくなる

したがって次の並走が自然である。

1. aggregate typed field の naming / migration cut を docs-only で切る
2. fixture authoring / elaboration は独立 task として進める

## この task で固定しないこと

- actual exporter API
- detached artifact 保存先と path policy
- current list / bool shape をいつ完全に置き換えるか
- object map vs row list を implementation でどこまで固定するか
- final parser syntax
- richer host interface の typed carrier 化
- multi-request scheduler
- `Approximate` / `Compensate`
- static analysis / theorem prover 側との boundary

## current L2 settled / OPEN

### current L2 settled

- aggregate 側に typed histogram / kind count を持たせるなら、bundle failure summary の再掲ではなく count 集約に留める
- field 名の最小候補は `bundle_failure_kind_counts` である
- row list shape が docs-only 段階では最小である
- docs-only の最小 migration cut は current bool/list anchor と additive に併存させることである

### OPEN

- actual exporter API
- replacement timing
- object map 形への将来移行可否
- future failure kind の増え方
- richer host interface typed 化との接続点
