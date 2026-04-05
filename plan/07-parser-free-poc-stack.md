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
- static reasons（detached static gate artifact の `checker_core` compare に限る）
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

ただし `static reasons` については current carrier が 2 種あることに注意する。

- detached static gate artifact の `checker_core.reasons` は actual compare してよい
- fixture の `expected_static.reasons` は explanatory note を兼ねるため、現時点では harness machine-check に上げない
- ただし future checker API の narrow migration としては、optional `expected_static.checked_reasons` を additive に置き、present のときだけ harness が actual static gate reasons を fail-closed compare してよい
- さらに current stable cluster tranche としては、stable inventory 8 kind に限って optional `expected_static.checked_reason_codes` を additive に置き、present のときだけ harness が actual static gate reason から導いた typed row を fail-closed compare してよい
- current tranche で supported なのは
  - `missing_lineage_assertion`
  - `lineage_assertion_edge_mismatch`
  - `declared_target_missing`
  - `declared_target_mismatch`
  - `capability_strengthens`
  - `missing_chain_head_option`
  - `missing_predecessor_option`
  - `missing_successor_option`
  であり、duplicate declaration cluster は引き続き non-promotion / fail-closed のままにする

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

## detached exporter consolidation の current understanding

detached exporter chain の current docs-only judgment は、`specs/examples/23-current-l2-detached-export-loop-consolidation.md` に 1 箇所へ集約する。

この時点の current understanding は次である。

- payload core は `RunReport` 由来である
- first exporter entry は `run_bundle` / `BundleRunReport`
- bundle-first detached artifact は
  - payload core
  - `bundle_context`
  - `detached_noncore`
  - human-facing explanation
  に分ける
- `host_plan_coverage_failure` は current detached artifact では aggregate-only に残す
- future typed bundle failure artifact の最小核は `failure.failure_kind`
- aggregate 側で typed 集約を持つなら最小 field 名候補は `bundle_failure_kind_counts`

これは current L2 の runtime semantics ではなく、PoC loop を継続実行しやすくするための docs-only / helper-boundary judgment である。

## non-production の detached export loop aid

current L2 では production exporter API はまだ固定しない。
ただし PoC loop の入口として、次の non-production helper を置いてよい。

- `crates/mir-semantics/examples/current_l2_emit_detached_bundle.rs`
  - `run_bundle` / `BundleRunReport` を起点に 1 bundle の detached artifact sketch を出す
- `crates/mir-semantics/examples/support/current_l2_detached_bundle_support.rs`
  - `FixtureBundle + BundleRunReport -> detached bundle artifact` の pure transform だけを shared support module として持つ
  - example private transform を repo 内 callable boundary へ落とす actual narrow cut だが、`lib.rs` / `harness.rs` の public API には上げない
- `crates/mir-semantics/examples/current_l2_emit_detached_aggregate.rs`
  - `run_directory` / `BatchRunSummary` を起点に aggregate detached artifact sketch を出す
- `crates/mir-semantics/examples/support/current_l2_detached_aggregate_support.rs`
  - `BatchRunSummary -> detached aggregate artifact` の pure transform だけを shared support module として持つ
  - example private transform を repo 内 callable boundary へ落とす actual narrow cut だが、`lib.rs` / `harness.rs` の public API には上げない
- `crates/mir-semantics/examples/current_l2_emit_static_gate.rs`
  - `load_fixture_from_path` / `static_gate_detailed` を起点に static gate artifact sketch を出す
- `crates/mir-semantics/examples/support/current_l2_static_gate_support.rs`
  - `CurrentL2Fixture + StaticGateResult -> static gate artifact` の pure transform だけを shared support module として持つ
  - first checker cut の local / structural floor を detached validation loop へ接続する actual narrow cut だが、`lib.rs` / `harness.rs` の public API には上げない
  - optional `detached_noncore.reason_codes` は helper-local / reference-only mirror に留め、exact-compare core や fixture-side typed carrier と混同しない
- `scripts/current_l2_diff_detached_artifacts.py`
  - payload core の exact-compare を最小で比較する
- `scripts/current_l2_diff_detached_aggregates.py`
  - aggregate artifact の `summary_core` exact-compare を最小で比較する
- `scripts/current_l2_diff_static_gate_artifacts.py`
  - static gate artifact の `checker_core` exact-compare を最小で比較する
  - `detached_noncore.reason_codes` は reference-only difference として表示してよい
- `scripts/current_l2_detached_loop.py`
  - bundle emitter、aggregate emitter、bundle diff helper、aggregate diff helper を detached validation loop として薄くつなぐ
  - 1 fixture export、aggregate summary export、2 bundle artifact compare、2 aggregate artifact compare を回しやすくする
  - `smoke-fixture` subcommand により、1 fixture の bundle export、optional reference compare、single-fixture aggregate smoke を 1 command で回せる
  - `smoke-try-rollback-locality` subcommand により、`e22` mismatch と `e21` frontier の representative pair を current default label つきで 1 command で回せる
  - `emit-static-gate` / `compare-static-gates` / `smoke-static-gate` により、static-only / malformed / underdeclared fixture でも static gate artifact の保存と compare を回せる
  - `smoke-same-lineage-checker` により、1 fixture の static gate artifact を保存し、same-lineage first checker spike をその artifact に対して回せる
  - `smoke-missing-option-checker` により、1 fixture の static gate artifact を保存し、missing-option second checker spike をその artifact に対して回せる
  - `smoke-capability-checker` により、1 fixture の static gate artifact を保存し、capability third checker spike をその artifact に対して回せる
  - compare helper の exit code `1` は informational difference として許容し、helper failure だけを non-zero で止める
- `scripts/current_l2_scaffold_fixture.py`
  - fixture authoring の boilerplate だけを `target/current-l2-fixture-scaffolds/` 下へ作る
  - runtime / static-only の違いと empty `.host-plan.json` sidecar の有無だけを扱い、expectation completion は authoring 側へ残す
- `scripts/current_l2_checked_reasons_assist.py`
  - detached static gate artifact の actual `checker_core.reasons` を読んで、fixture-side `expected_static.checked_reasons` の copyable suggestion を display-only で返す
  - fixture JSON の自動更新や `checked_reasons = []` の一括補完は行わない
- `scripts/current_l2_reason_codes_assist.py`
  - detached static gate artifact の helper-local / reference-only `detached_noncore.reason_codes` を読んで、future typed carrier 候補 row を display-only で返す
  - fixture-side `expected_static.checked_reason_codes` が current tranche で present のときは、その current row と actual suggestion の一致も display-only に確認できる
  - unsupported legacy fixture-side typed field を見つけたら fail-closed に止まる
  - fixture JSON の自動更新は行わない
- `scripts/current_l2_reason_code_readiness.py`
  - static-only fixture corpus を横断し、`checked_reasons` adoption と `detached_noncore.reason_codes` suggestion availability を batch で display-only 要約する
  - current stable cluster tranche の `checked_reason_codes` adoption 数も同じ scan で観察してよい
  - coexistence scan として、stable coexistence anchor 数、`checked_reason_codes` はあるが `checked_reasons` が無い fixture 数、typed row mismatch 数も同じ summary で見てよい
  - first checker cut readiness の companion baseline として、stable kind を same-lineage / capability / missing-option の checker cluster に roll-up した coverage count も同じ summary で見てよい
  - stable cluster と duplicate cluster の current split を tranche 単位で観察する authoring aid に留め、typed carrier actualization や detached aggregate 永続化は行わない
- `scripts/current_l2_same_lineage_checker.py`
  - first checker cut の actual first spike として、fixture-side `checked_reason_codes` と static gate artifact の actual reason rows を読み、same-lineage family だけを narrow compare する helper-local checker spike
  - public checker API ではなく、detached validation loop から smoke する non-production helper に留める
- `scripts/current_l2_missing_option_checker.py`
  - second checker spike として、fixture-side `checked_reason_codes` と static gate artifact の actual reason rows を読み、missing-option family だけを narrow compare する helper-local checker spike
- `scripts/current_l2_capability_checker.py`
  - third checker spike として、fixture-side `checked_reason_codes` と static gate artifact の actual reason rows を読み、capability family だけを narrow compare する helper-local checker spike
- `scripts/current_l2_family_checker_support.py`
  - same-lineage / missing-option / capability の 3 checker spike で重複する parser / filter / status / stdout rendering をまとめる non-production support module
  - family facade script と detached loop wrapper command 名は残し、generic checker-side shared entry にはしない
  - public checker API 化はまだ行わない

これらは current helper stack の public behavior を置き換えない。
実行補助であり、production API や final serialization contract として扱わない。

current non-production default candidate としては、artifact root を `target/current-l2-detached/` に置く。
ただしこれは final path policy ではなく、repo 相対で generated artifact を散らさず、`.gitignore` 既存境界の内側で loop を回しやすくするための暫定運用である。

aggregate 側 actual narrow cut としては、

- `aggregates/<run-label>/batch-summary.detached.json`
- `bundle_failure_kind_counts`
- `bundle_failure_kind_counts_scope = "migrated-kinds-only"`
- current `host_plan_coverage_failures` list の additive coexistence
- aggregate emitter 本体の private transform を `examples/support/current_l2_detached_aggregate_support.rs` へ切り出し、non-production helper 内でだけ再利用できる repo 内 callable boundary を持つ

までを non-production helper で扱ってよい。
ただし actual exporter API は引き続き OPEN である。

bundle 側 actual narrow cut としては、

- `bundles/<run-label>/<fixture-stem>.detached.json`
- `bundle_context`
- `payload_core`
- `detached_noncore.steps_executed`
- bundle emitter 本体の private transform を `examples/support/current_l2_detached_bundle_support.rs` へ切り出し、non-production helper 内でだけ再利用できる repo 内 callable boundary を持つ

までを non-production helper で扱ってよい。
ただし actual exporter API は引き続き OPEN である。

current detached validation loop では、aggregate compare helper をさらに narrow に足してよい。

- exact-compare core
  - `summary_core`
- reference-only
  - `aggregate_context`
  - `detached_noncore`
- wrapper convenience
  - `compare-aggregates <left-run-label> <right-run-label>`

ただしこれは production compare API を意味しない。

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
その naming と migration cut を docs-only でさらに narrow に切るなら、field 名は `bundle_failure_kind_counts` が最小候補である。

- `bundle_` prefix は current `BatchRunSummary` が `discovery_failures` と `bundle_failures` を分けている helper boundary と整合する
- `failure_kind` は bundle failure artifact 側の `failure.failure_kind` と接続しやすい
- `counts` は aggregate の coarse summary role に合い、per-bundle summary 再掲を誘発しにくい

ただし current code の list / bool shape をただちに置き換える判断はしない。
docs-only の最小 migration cut は、`host_plan_coverage_failures` list と `BatchBundleOutcome::Failed.host_plan_coverage_failure` bool を compatibility anchor として残したまま、aggregate 側に `bundle_failure_kind_counts` を additive に併存させる形である。
置換時期と actual exporter API は引き続き OPEN に残す。

field-name / migration-cut refinement の正本は `specs/examples/22-current-l2-host-plan-coverage-failure-aggregate-histogram-migration.md` に置く。
storage / aggregate API refinement の正本は `specs/examples/24-current-l2-detached-export-storage-and-aggregate-api.md` に置く。

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
