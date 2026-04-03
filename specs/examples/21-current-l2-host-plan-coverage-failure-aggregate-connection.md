# examples/21 — current L2 host plan coverage failure aggregate connection

## 目的

この文書は、current L2 parser-free PoC 基盤で bundle failure artifact 側の `failure.failure_kind` discriminator-only schema を将来導入するとしたら、`BatchRunSummary` aggregate export がその typed failure をどこまで吸うべきかを narrow scope で整理する。

ここで固定するのは docs-only の boundary judgment だけである。production exporter 実装、actual exporter API、runtime semantics、parser grammar、failure family は固定しない。

## 前提

- current detached artifact の current judgment は次である。
  - detached success artifact では `host_plan_coverage_failure` を aggregate-only に残す。
  - 将来 typed carrier に昇格させる最小 layer は bundle failure artifact 側である。
  - その最小 typed core は `failure.failure_kind` の discriminator-only である。
- `RunReport` payload core と bundle-first success artifact の split は変えない。
- `fixture authoring / elaboration` は独立 bottleneck として残る。
- richer host interface は後段に残す。

## current code anchor

current code では、`host_plan_coverage_failure` は次のように materialize される。

- `RunReport` の field ではない。
- `BundleRunReport` の field でもない。
- `BatchBundleOutcome::Failed { host_plan_coverage_failure: bool }` の failure bit として現れる。
- `BatchRunSummary.host_plan_coverage_failures` は `BundleExecutionFailure` の配列として集約する。
- `batch_summary_from_discovery()` が error text からこれを分類して立てる。
- tests では summary / batch report 側の failure classification だけでなく、harness layer の raw error wording も固定している。

したがって current code の意味としては、これは success payload ではなく、**bundle failure classification が batch aggregate に持ち上がったもの**である。
ただし typed migration の影響範囲は aggregate export だけではなく、current harness error wording anchor にも及ぶ。

## 比較する 3 案

### 1. current counter / list 維持

aggregate は current code と同様に、`host_plan_coverage_failures` の list と `BatchBundleOutcome::Failed.host_plan_coverage_failure` の bool を維持する。

#### 利点

- current helper stack と完全に一致する。
- exporter 実装前に typed aggregate shape を増やさなくてよい。
- bundle-first failure artifact と aggregate export を独立に進められる。

#### 欠点

- bundle failure artifact 側で `failure.failure_kind` を持つ将来像と直接つながらない。
- `host_plan_coverage_failure` という専用名に寄りすぎており、typed failure 集約としては coarse すぎる。
- batch export のままでも per-bundle list と bool の二重表現が残る。

#### 評価

current code / current detached artifact の **現状維持**としては妥当である。
ただし future typed bundle failure artifact を aggregate export がどう吸うべきか、という問いへの答えとしては narrow judgment が足りない。

### 2. aggregate 側に typed failure histogram / kind count を持つ

aggregate は bundle failure artifact 側の `failure.failure_kind` を集約し、`failure_kind` ごとの count だけを保持する。

#### 最小 shape の例

```json
{
  "artifact_kind": "batch-aggregate",
  "typed_failure_histogram": [
    {
      "failure_kind": "host-plan-coverage-failure",
      "count": 1
    }
  ]
}
```

#### 利点

- batch summary の coarse 役割を壊しにくい。
- bundle failure artifact の typed core をそのまま集約できる。
- `bundle_context` や detached non-core explanation を aggregate 側へ持ち込まずに済む。
- exact-compare payload core を汚さない。
- future richer host interface 側で failure kind が増えても、aggregate は count 集約に留められる。

#### 欠点

- current code にはまだこの histogram carrier が無い。
- aggregate が typed shape を持つ時点で、current list / bool との migration 方針を別 task で決める必要がある。

#### histogram と flat kind list の比較

typed aggregate の内部でも、flat kind list より histogram の方が最小である。

- flat kind list
  - 例: `["host-plan-coverage-failure", "host-plan-coverage-failure"]`
  - multiplicity は持てるが、summary より log に近く、count を読むたびに再集計が要る。
  - bundle failure artifact が別に存在する前提では、aggregate としての利点が薄い。
- histogram / kind count
  - 例: `[{ "failure_kind": "host-plan-coverage-failure", "count": 2 }]`
  - coarse summary の役割に合う。
  - future failure kind を先取りしすぎず、known kind の count だけを増やせる。

#### 評価

**typed aggregate を持たせるなら、この histogram / kind count が最小**である。

### 3. aggregate 側に bundle failure summary を薄く再掲する

aggregate に、bundle failure artifact を薄く再掲する summary list を持たせる。

```json
{
  "artifact_kind": "batch-aggregate",
  "bundle_failure_summaries": [
    {
      "fixture_id": "e1-place-atomic-cut",
      "failure_kind": "host-plan-coverage-failure"
    }
  ]
}
```

#### 利点

- aggregate だけ読んでも per-bundle failure をざっくり追える。
- detached artifact を 1 つだけ見たい運用には直感的である。

#### 欠点

- batch summary の coarse 役割を壊しやすい。
- bundle failure artifact と責務競合し、bundle-level identity を二重保持する。
- `bundle_context` を aggregate 側へ薄く再掲する圧力が生まれる。
- detached non-core explanation や short note を一緒に載せたくなり、aggregate が肥大化しやすい。

#### 評価

comparison 対象としては有益だが、**current L2 の next narrow step としては採らない**。

## aggregate 側が typed に持つべき最小情報

### 持ってよいもの

- `failure_kind`
- `count`

これは typed histogram / kind count に留める。

### aggregate 側へ持ち込まないもの

- `bundle_context`
- `fixture_id`
- `fixture_path`
- `host_plan_path`
- `runtime_requirement`
- detached non-core explanation
- `must_explain`

理由:

- これらは bundle failure artifact か human-facing explanation の責務である。
- aggregate へ持ち込むと、coarse summary と per-bundle artifact の責務が重なる。

## `failure.failure_kind` を aggregate へどうつなぐか

future typed bundle failure artifact を導入するとしても、aggregate 側は **`failure.failure_kind` を直接再掲する list** にせず、count 集約に戻す方が current helper boundary と整合する。

つまり接続は、概念上は次のように読むのが自然である。

```text
bundle failure artifact:
  failure.failure_kind = "host-plan-coverage-failure"

BatchRunSummary aggregate export:
  typed_failure_histogram["host-plan-coverage-failure"] += 1
```

ここで aggregate は bundle failure artifact の存在を前提に summary 化するだけであり、bundle failure artifact の代替にはならない。

## current judgment

### current repo の next narrow step として採る案

- **aggregate 側に持たせる typed 集約は histogram / kind count までに留める。**

これは「今すぐ implementation する」という意味ではない。
docs-only boundary judgment としては、bundle failure artifact の typed core を aggregate が吸うとしても、それ以上に厚くしないのが最小である。

### 次点案

- current counter / list 維持を続け、typed aggregate 自体はまだ導入しない。

これは implementation timing としては十分ありうる。
ただし docs-only の connection judgment としては、typed aggregate を持たせるなら histogram が最小だと先に言える。

### 今回は採らない案

- typed failure flat kind list
- bundle failure summary の薄い再掲

## `fixture authoring / elaboration` との関係

`fixture authoring / elaboration` は引き続き独立 bottleneck である。

- aggregate connection を narrow に切っても、fixture 記述作業や elaboration cost そのものは減らない。
- ただし typed histogram があれば、多数ケースを回したときに「どの kind が何件出たか」を coarse に比較しやすくなる。

したがって、次の切り方が自然である。

1. bundle failure artifact と aggregate export の connection boundary を docs-only で整える
2. fixture authoring / elaboration は独立 task として並走させる

## この task で固定しないこと

- actual exporter API
- detached artifact 保存先と path policy
- current `host_plan_coverage_failures` list を histogram で置き換えるか併存させるか
- richer host interface の typed carrier 化
- final parser syntax
- multi-request scheduler
- `Approximate` / `Compensate`
- static analysis / theorem prover 側との boundary

## current L2 settled / OPEN

### current L2 settled

- future typed bundle failure artifact の最小核は `failure.failure_kind` discriminator-only である。
- aggregate export が typed bundle failure を吸うとしても、持たせる typed 集約は histogram / kind count までが最小である。
- bundle failure summary の薄い再掲は採らない。
- `bundle_context`、detached non-core explanation、`must_explain` は aggregate へ持ち込まない。

### OPEN

- actual exporter API
- current list / bool shape と typed histogram の migration timing
- histogram / kind count の field 名と docs-only migration cut の refinement
- detached artifact 保存先と path policy
- `failure_kind` を将来どこまで細分化するか
- richer host interface typed 化との接続点

field 名と migration cut の narrow refinement は `specs/examples/22-current-l2-host-plan-coverage-failure-aggregate-histogram-migration.md` に切り出す。
