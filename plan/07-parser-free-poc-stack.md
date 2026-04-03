# plan/07 — parser-free PoC stack

## 目的

current L2 の parser-free PoC stack は、representative examples を parser なしで machine-check するための最小 execution / verification infrastructure である。
ここでは各 layer の public behavior と thin delegation の境界を揃えて記録する。

## stack の全体像

### artifact / helper の順序

1. AST fixture schema
2. evaluation state schema
3. step semantics
4. oracle API
5. parser-free minimal interpreter
6. host harness
7. host plan sidecar loader
8. bundle loader
9. batch runner
10. selection helper
11. selection profile helper
12. named profile catalog

## 実行 call chain

current helper stack の代表的な呼び出しは次である。

```text
run_directory_named_profile
  -> run_directory_profiled
    -> discover_bundles_in_directory
    -> select_bundles_from_request
    -> batch_summary_from_discovery
      -> run_bundle
        -> FixtureHostStub::run_fixture
          -> run_to_completion
```

この call chain で、各 layer は自分の public behavior だけを持ち、下位の execution / comparison をなるべく再利用する。

## 各 layer の public behavior と thin delegation

| layer | public behavior | thin delegation に留めるもの |
|---|---|---|
| AST fixture schema | program / expected_static / expected_runtime / expected_trace_audit を machine-readable に持つ | final parser syntax の固定 |
| evaluation state schema | parser-free 実行に必要な最小 state 粒度 | production object model |
| step semantics | 1-step rule と terminal outcome の導出 | full scheduler |
| oracle API | `PredicateOracle` / `EffectOracle` の最小境界 | richer host API |
| parser-free minimal interpreter | static gate、`step_once`、`run_to_completion` | production runtime |
| host harness | fixture ごとの declarative host plan による oracle stub | production host interface |
| host plan sidecar | predicate/effect/commit/narrative override を asset 化 | manifest 化 |
| bundle loader | fixture + sidecar を 1 bundle として解決し、1 bundle を run する | directory summary |
| batch runner | directory discovery と bundle 群の一括実行 summary | selection / alias |
| selection helper | `runtime-only` / `static-only` / `single-fixture` で bundle 群を絞る | profile 名 / alias |
| selection profile helper | primitive selection を request と profile 名で束ねる | alias catalog |
| named profile catalog | human-friendly alias を `SelectionRequest` へ解決する | sidecar discovery、runtime/static classification |

## parser-free minimal interpreter の役割

### public behavior

- fixture load
- static gate
- `EvaluationState`
- `step_once`
- `run_to_completion`
- `success` / `explicit_failure` / `Reject` / static-only stop の照合

### まだやらないこと

- full parser
- full production runtime
- distributed scheduler
- `Approximate` / `Compensate`

## host harness と host plan sidecar

### host harness

- `PredicateOracle` / `EffectOracle` を declarative plan で stub 化する
- uncovered oracle call は fail-closed にする
- overlap rule は reject する

### sidecar

- `.host-plan.json`
- runtime fixture では必須
- static-only fixture では不要
- machine-readable asset だが production manifest ではない

## bundle / batch / selection / profile / catalog

### bundle

- fixture 本体
- `expected_static`
- `expected_runtime`
- `expected_trace_audit`
- optional sidecar

### batch

- fixture directory を discovery して bundle 群として実行
- total / runtime / static-only / passed / failed / discovery failure / host-plan coverage failure を返す

### selection

- `runtime-only`
- `static-only`
- `single-fixture`

### profile

- optional scope
- optional single-fixture selector
- `profile_name`

### named profile catalog

- current aliases:
  - `smoke-runtime`
  - `smoke-static`
  - `runtime-e3`
  - `static-e4`
- hard-coded table を維持
- machine-readable catalog externalization は比較止まり

## current named profile catalog の boundary

### code 側

- `ProfileCatalog::aliases()` / `resolve()` は single source of truth 化済み
- hard-coded preset table が正本

### docs 側

- concrete alias prose は `specs/examples/13-current-l2-profile-catalog.md` に寄せる

### tests 側

- integration tests は literal `resolved_request` と unknown alias failure を public behavior coverage として持つ
- selected counts / concrete fixture shape は profile-layer tests に寄せる
- internal tests は preset table wiring の整合だけを見る

## machine-check と human-facing explanation の境界

### machine-check に残すもの

- static verdict
- runtime final outcome
- event kinds
- formal non-admissible metadata
- short narrative explanations
- batch-layer の `host_plan_coverage_failure`
- selected bundle counts や concrete fixture shape の一部
- `resolved_request`

### prose に残すもの

- `must_explain`
- long-form audit explanation
- static verdict reason の長文
- なぜ current L2 でその helper boundary を採るかの比較理由

### detached artifact に出してよいが core にしないもの

- `steps_executed`
- coverage explanation
- host-plan explanation
- auxiliary counters / summaries

current L2 では、これらは後比較や後解析には有益だが、interpreter / helper refactor で揺れやすいため exact-compare core へは上げない。

## `must_explain` の扱い

`must_explain` は current L2 では machine-check に上げない。
これは bundle / batch / selection / profile / catalog のどの layer でも一貫している。

## detached trace / audit artifact の docs-only boundary

current L2 では、detached trace / audit serialization そのものはまだ production 実装しない。
ただし docs-only minimal schema としては、次の 3 群を分けるところまでは固定してよい。

1. exact-compare core
   - `static_verdict`
   - `entered_evaluation`
   - `terminal_outcome`
   - `event_kinds`
   - formal `non_admissible_metadata`
   - short `narrative_explanations`
   - batch-layer の `host_plan_coverage_failure`
2. detached artifact に出してよいが core にはしないもの
   - `steps_executed`
   - coverage explanation
   - host-plan explanation
   - auxiliary counters / summaries
3. human-facing explanation に残すもの
   - `must_explain`
   - long-form audit
   - why-this-is-good/bad の説明

この docs-only boundary は `specs/examples/16-current-l2-detached-trace-audit-artifact-schema.md` を正本とする。

## detached exporter entry の current judgment

detached artifact exporter を narrow に始める comparison では、payload core と entry boundary を分けて考える。

- payload core
  - `RunReport` に最も近い。
- first exporter entry
  - `run_bundle` / `BundleRunReport` を採るのが最小である。
- second stage
  - `BatchRunSummary` export は bundle-level exporter の後段に回す。

この判断は、bundle が `fixture + sidecar` の自然単位であり、payload core と detached non-core context を分けたまま helper boundary を壊しにくいためである。
正本は `specs/examples/17-current-l2-detached-exporter-entry-comparison.md` に置く。

## bundle-first detached payload/context split

bundle-first exporter をさらに narrow に切る current understanding では、artifact の内部も次の 4 層に分ける。

1. payload core
   - `RunReport` 由来
   - `static_verdict`
   - `entered_evaluation`
   - `terminal_outcome`
   - `event_kinds`
   - formal `non_admissible_metadata`
   - short `narrative_explanations`
2. `bundle_context`
   - `fixture_id`
   - `fixture_path`
   - `host_plan_path`
   - `runtime_requirement`
3. detached non-core
   - `steps_executed`
   - auxiliary counters
   - coverage explanation
   - host-plan explanation
4. human-facing explanation
   - `must_explain`
   - long-form audit
   - why-this-is-good/bad の prose

`host_plan_coverage_failure` は current code では batch / per-bundle failure classification であり、`RunReport` payload core でも `FixtureBundle` context でもない。
そのため bundle-first artifact へは入れず、aggregate-only に残すのが current L2 の最小 judgment である。
正本は `specs/examples/18-current-l2-bundle-first-detached-payload-context-split.md` に置く。

## `host_plan_coverage_failure` の future typed placement

current detached artifact では `host_plan_coverage_failure` を aggregate-only に残す。
ただし将来 typed carrier に昇格させるなら、最も自然な layer は **bundle failure artifact 側**である。

理由は次の通りである。

- current code ですでに `BatchBundleOutcome::Failed { host_plan_coverage_failure: bool }` として per-bundle failure classification がある
- `RunReport` payload core を汚さない
- `bundle_context` と detached non-core の責務を崩さない
- `BatchRunSummary` はその typed bundle failure を集約する後段に留められる

その次の docs-only refinement としては、bundle failure artifact の typed core を **`failure_kind` discriminator だけ**に留めるのが最小である。

- artifact 全体では `bundle_context` を別 section として持ってよい
- ただし typed carrier 自体には context reference や short note を抱かせない
- short coverage note は必要なら detached non-core の後段 task に回す

placement judgment の正本は `specs/examples/19-current-l2-host-plan-coverage-failure-placement.md` に置く。
schema refinement の正本は `specs/examples/20-current-l2-host-plan-coverage-failure-bundle-failure-artifact-schema.md` に置く。
aggregate export との connection comparison の正本は `specs/examples/21-current-l2-host-plan-coverage-failure-aggregate-connection.md` に置く。

typed bundle failure artifact を aggregate export が吸うとしても、`BatchRunSummary` は coarse summary に留まるべきである。
そのため current understanding では、aggregate 側に持たせる typed 集約は `failure_kind` ごとの histogram / kind count までを最小とし、bundle failure summary の薄い再掲は採らない。
current code の list / bool shape を histogram で置き換えるか併存させるかは、actual exporter API を切る task まで OPEN に残す。

## current L2 settled / OPEN

### current L2 settled

- parser-free PoC stack の基本 call chain
- bundle / batch / selection / profile / catalog の順序
- hard-coded named profile catalog
- helper layer ごとの public behavior / thin delegation の分離
- detached trace / audit artifact の docs-only minimal grouping
- detached exporter entry の bundle-first judgment
- bundle-first detached exporter の payload/context split

### OPEN

- production detached trace / audit serialization
- richer host interface
- machine-readable catalog asset / manifest
- multi-request scheduler
- `Approximate` / `Compensate`
