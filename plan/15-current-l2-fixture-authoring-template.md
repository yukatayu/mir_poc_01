# plan/15 — current L2 fixture authoring template

## 目的

この文書は、current L2 parser-free PoC 基盤に新しい fixture を 1 本追加するときの最小テンプレートを与える。

ここでの目的は fixture authoring / elaboration を完全自動化することではなく、

- 何を必ず揃えるか
- runtime fixture と static-only fixture で何が違うか
- detached artifact loop に入った後に何を追加で見るか
- どこから先が host interface / exporter / batch responsibility か

を、task ごとに拾い直さなくて済む形で固定することにある。

## fixture authoring の最小 checklist

新しい current L2 fixture を 1 本足すときは、最低でも次を確認する。

1. AST fixture
2. runtime / static-only の判定
3. host plan sidecar の有無
4. `expected_static`
5. `expected_runtime`
6. `expected_trace_audit`
7. named profile / selection への影響
8. detached artifact loop に入れるなら比較観点の追加

## 1. AST fixture

### 必須

- `schema_version`
- `fixture_id`
- `source_example_id`
- `program`
- `expected_static`
- `expected_runtime`
- `expected_trace_audit`

### 書くときの確認点

- current L2 parser-free AST fixture schema に乗っているか
- new semantics や new failure class を勝手に足していないか
- `perform` / option chain / `try` / `fallback` / `atomic_cut` の current reading を越えていないか

## 2. runtime fixture と static-only fixture の違い

### static-only fixture

- `expected_runtime.enters_evaluation = false`
- `.host-plan.json` sidecar は不要
- static gate verdict と reasons が主な比較軸になる
- detached artifact loop に入れても、payload core の中心は `static_verdict` と `entered_evaluation = false` になる

### runtime fixture

- `expected_runtime.enters_evaluation = true`
- `.host-plan.json` sidecar が必須
- `terminal_outcome`、`event_kinds`、formal `non_admissible_metadata`、short `narrative_explanations` まで比較する
- detached artifact loop に入れるときは `bundle_context.host_plan_path` と `runtime_requirement` が必須になる

## 3. host plan sidecar の有無

### sidecar が必要なとき

- runtime fixture である
- predicate / effect oracle call が入る
- current host harness の fail-closed behavior を通す必要がある

### sidecar が不要なとき

- static-only fixture で evaluation に入らない

### 注意

- sidecar を足すかどうかは fixture authoring の責務であり、exporter や batch helper の責務ではない
- `.host-plan.json` の placement / loading は current helper stack の既存 boundary を使う

## 4. `expected_static`

最低でも次を揃える。

- `verdict`
- `reasons`

確認点:

- `valid` / `malformed` / `underdeclared` の current gate judgment と一致しているか
- malformed / underdeclared を runtime 側でごまかしていないか

## 5. `expected_runtime`

最低でも次を揃える。

- `enters_evaluation`
- `final_outcome`
- `notes`

確認点:

- runtime fixture なら `final_outcome` が current L2 reading と一致しているか
- static-only fixture なのに `enters_evaluation = true` になっていないか
- `notes` は machine-check core ではなく、補助説明であることを混同していないか

## 6. `expected_trace_audit`

最低でも次を揃える。

- `event_kinds`
- `non_admissible_metadata`
- `narrative_explanations`
- `must_explain`

### machine-check に残すもの

- `event_kinds`
- formal `non_admissible_metadata`
- short `narrative_explanations`

### human-facing に残すもの

- `must_explain`

ここでは `must_explain` を current L2 policy に従って machine-check に上げない。

## 7. named profile / selection への影響

新しい fixture を足したときは、次を確認する。

- `runtime-only` / `static-only` の既存 selection に影響するか
- `single-fixture` selector で明示的に拾う必要があるか
- current named profile catalog
  - `smoke-runtime`
  - `smoke-static`
  - `runtime-e3`
  - `static-e4`
  の意味を変えるか

### 原則

- 既存 alias の意味を変える場合は helper stack / tests / docs mirror を同 task で更新する
- 単に fixture 数が増えるだけなら alias catalog をむやみに増やさない

## 8. detached artifact loop に入った後の追加観点

fixture を detached artifact loop に入れるなら、fixture expectation だけでなく次も見る。

### payload core

- `static_verdict`
- `entered_evaluation`
- `terminal_outcome`
- `event_kinds`
- formal `non_admissible_metadata`
- short `narrative_explanations`

### `bundle_context`

- `fixture_id`
- `fixture_path`
- `host_plan_path`
- `runtime_requirement`

### `detached_noncore`

- `steps_executed`
- optional coverage explanation
- optional host-plan explanation

### 比較 helper で見ないもの

- `must_explain`
- long-form audit
- why-this-is-good/bad の prose

## 9. どこから先が fixture authoring で、どこから先が別責務か

| 境界 | fixture authoring の責務 | 別責務 |
|---|---|---|
| AST shape | fixture JSON を current schema に合わせる | final parser syntax |
| runtime/static 判定 | `expected_runtime.enters_evaluation` と sidecar 必要性を決める | production host interface |
| expectation 記述 | `expected_static` / `expected_runtime` / `expected_trace_audit` を揃える | exporter API / detached path policy |
| sidecar 記述 | `.host-plan.json` を必要な runtime fixture に付ける | richer host interface typed carrier |
| detached artifact 比較 | payload core / `bundle_context` / `detached_noncore` のどこを見るか決める | batch aggregate export の final shape |

## 10. 新しい fixture を足すときの実務テンプレート

### static-only fixture

1. fixture JSON を追加する
2. `expected_static` / `expected_runtime` / `expected_trace_audit` を揃える
3. static-only selection と representative catalog の更新要否を確認する
4. 必要なら detached artifact を export して payload core を確認する

### runtime fixture

1. fixture JSON を追加する
2. `.host-plan.json` sidecar を追加する
3. `expected_static` / `expected_runtime` / `expected_trace_audit` を揃える
4. bundle-first exporter で 1 bundle artifact を出す
5. diff helper で exact-compare core を比較する
6. profile / selection への影響を確認する

## 11. detached exporter loop との接続

current detached export loop では、fixture は次の単位で PoC を回す。

1. fixture を追加 / 更新する
2. `run_bundle` 起点の tiny exporter で artifact を出す
3. minimal diff helper で payload core を比較する
4. 必要なら detached_noncore を参考表示として読む
5. fixture を追加してまた回す

この loop は fixture authoring を完全自動化しない。
ただし、「1 本ずつ狭く正確に回す」から「artifact を保存し、比較し、また 1 本足す」へ進む最小入口としては十分である。

## 12. 依然として OPEN のもの

- actual exporter API
- detached artifact 保存先と path policy
- actual elaboration helper
- richer host interface の typed carrier 化
- final parser syntax
- multi-request scheduler
- `Approximate` / `Compensate`

