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
9. detached artifact 保存先 / run label / compare 手順の確認
10. boilerplate を scaffold helper で起こすか、手で直接書き始めるかの判断

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
- detached validation loop continuation では、runtime bundle artifact と別に static gate artifact を保存し、`checker_core.static_verdict` / `checker_core.reasons` を compare してよい

### runtime fixture

- `expected_runtime.enters_evaluation = true`
- `.host-plan.json` sidecar が必須
- `terminal_outcome`、`event_kinds`、formal `non_admissible_metadata`、short `narrative_explanations` まで比較する
- detached artifact loop に入れるときは `bundle_context.host_plan_path` と `runtime_requirement` が必須になる

### profile-targeted run

- fixture 自体は runtime fixture か static-only fixture のどちらかである
- ただし detached validation loop に載せるときは、その fixture が
  - `runtime-only`
  - `static-only`
  - `single-fixture`
  - named profile
  のどれで拾われる想定かを別に確認する
- profile-targeted run は fixture authoring の主目的ではないが、fixture 追加が profile の意味を変える場合は authoring task 側で mirror 更新要否を確認する

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

### current non-production 保存先候補

- current detached validation loop の non-production default candidate は `target/current-l2-detached/`
- bundle artifact は `bundles/<run-label>/<fixture-stem>.detached.json` を基本にする
- compare は explicit artifact path を直接渡すか、thin wrapper が `artifact_root + run_label + fixture_stem` から path を導出する
- final path policy ではないので、repo 追跡下の固定保存先と誤読しない

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
| profile-targeted run | profile / selection への影響を確認する | alias catalog の final 設計、profile helper の public API |

## 10. 新しい fixture を足すときの実務テンプレート

### scaffold helper を使ってよい範囲

- `scripts/current_l2_scaffold_fixture.py` は non-production helper である
- default candidate は `target/current-l2-fixture-scaffolds/`
- helper が作ってよいのは
  - top-level required carrier
  - runtime / static-only の別
  - runtime fixture 用の empty `.host-plan.json` sidecar
  までである
- helper が作ってはいけないのは
  - completed `program`
  - completed `expected_static`
  - completed `expected_runtime`
  - completed `expected_trace_audit`
  である

scaffold を使うときは、まず `target/` 下に skeleton を出し、その後に review 可能な hand-written fixture へ詰める。

### static-only fixture

1. fixture JSON を追加する
2. `expected_static` / `expected_runtime` / `expected_trace_audit` を揃える
3. static-only selection と representative catalog の更新要否を確認する
4. detached validation loop に入れるなら `emit-static-gate` で static gate artifact を保存する
5. 必要なら既存 artifact と `checker_core` を比較する
6. runtime artifact も参考に見たいときだけ `emit-fixture` を併用する
7. 1 command でまとめたいときは `scripts/current_l2_detached_loop.py smoke-static-gate` を non-production convenience として使ってよい
8. directory-level の差を見たいときは `emit-aggregate` のあと `compare-aggregates` で `summary_core` を比較する

### runtime fixture

1. fixture JSON を追加する
2. `.host-plan.json` sidecar を追加する
3. `expected_static` / `expected_runtime` / `expected_trace_audit` を揃える
4. host plan sidecar が current host harness の fail-closed rule を満たすか確認する
5. bundle-first exporter で 1 bundle artifact を出す
6. diff helper で exact-compare core を比較する
7. directory-level の差も見たいときは `emit-aggregate` と `compare-aggregates` を使う
8. profile / selection への影響を確認する
9. current non-production convenience としては、`smoke-fixture` で
   - target fixture artifact
   - optional reference fixture compare
   - full directory vs single-fixture aggregate smoke
   を 1 command で回してよい

runtime fixture を最初から全部手で書く代わりに、scaffold helper で骨組みを起こしてから埋めてもよい。

### profile-targeted run

1. fixture を追加 / 更新する
2. その fixture がどの selection / named profile に入るべきかを確認する
3. detached bundle artifact 自体は bundle-first loop で確認する
4. aggregate summary の差を見たいときは aggregate artifact を保存して compare する
5. selection / profile の妥当性は batch / profiled run 側で確認する
6. profile alias の意味が変わるなら docs / tests / code mirror を同 task で更新する

## 11. detached exporter loop との接続

current detached export loop では、fixture は次の単位で PoC を回す。

1. fixture を追加 / 更新する
2. `scripts/current_l2_detached_loop.py emit-fixture` か `compare-fixtures` で bundle artifact を保存する
3. minimal diff helper で payload core を比較する
4. 必要なら `bundle_context` と detached_noncore を reference-only として読む
5. directory-level の smoke を取りたいときは `emit-aggregate` で aggregate summary も保存する
6. aggregate summary 同士を比べたいときは `compare-aggregates` で `summary_core` を比較する
7. batch / profile の論点があるときだけ別に `run_directory_profiled` / named profile 側を見る
8. fixture を追加してまた回す

この loop は fixture authoring を完全自動化しない。
ただし、「1 本ずつ狭く正確に回す」から「artifact を保存し、比較し、また 1 本足す」へ進む最小入口としては十分である。

## 12. fixture authoring と exporter / batch / host interface の責務境界

### fixture authoring の責務

- fixture JSON と `.host-plan.json` sidecar を揃える
- `expected_static` / `expected_runtime` / `expected_trace_audit` を current semantics に合わせる
- detached bundle artifact を出して payload core を確認する
- profile / selection への影響有無を確認する

### exporter / batch の責務

- bundle artifact の保存 path を導く
- batch aggregate export を coarse summary として扱う
- `bundle_failure_kind_counts` のような aggregate typed field を fixture JSON に押し込まない

### host interface 側へ送るもの

- typed uncovered-call carrier
- preflight coverage analysis
- richer host-plan explanation carrier

これらは fixture authoring task ではなく、後段の richer host interface workstream で扱う。

## 13. 依然として OPEN のもの

- actual exporter API
- detached artifact 保存先と path policy
- actual elaboration helper
- richer host interface の typed carrier 化
- final parser syntax
- multi-request scheduler
- `Approximate` / `Compensate`
