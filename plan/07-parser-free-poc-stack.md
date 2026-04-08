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

## parser boundary 側の current non-production helper

current parser-free PoC stack の主線は依然として fixture-first だが、
parser boundary の next narrow step として、
test-only / private な stage 1 parser spike first tranche が actualize 済みである。

### current actualized helper

- `crates/mir-ast/tests/support/current_l2_stage1_parser_spike_support.rs`
- `crates/mir-ast/tests/current_l2_stage1_parser_spike.rs`
- `crates/mir-ast/tests/support/current_l2_stage3_admit_slot_spike_support.rs`
- `crates/mir-ast/tests/current_l2_stage3_admit_slot_spike.rs`
- `crates/mir-ast/tests/support/current_l2_stage3_multiline_attachment_spike_support.rs`
- `crates/mir-ast/tests/current_l2_stage3_multiline_attachment_spike.rs`
- `crates/mir-ast/tests/support/current_l2_stage3_request_clause_suite_spike_support.rs`
- `crates/mir-ast/tests/current_l2_stage3_request_clause_suite_spike.rs`

### current scope

- input surface は inline test strings
- declaration-side guard slot は `decl_guard_slot.surface_text` として保持
- compare は lowered fixture-subset compare に留める
- `e4` / `e7` の two-fixture pair と guard-slot retention smoke を cargo test で回す
- stage 3 admit-slot branch の success-side first tranche として、`e3` 由来 option / chain subset の lowered fixture-subset compare と `decl_admit_slot.surface_text` retention smoke を cargo test で回す
- stage 3 admit-slot branch の malformed-source first tranche として
  - `missing declaration-side admit slot payload`
  - `request head is outside stage 3 admit-slot first tranche`
  の 2 件を substring compare で見る
- stage 3 admit-slot branch の次段 sequencing では、request-local clause spillover より先に fixture-side `OptionDecl.admit` handoff line を docs-only で比較する
- ただし current phase では、fixture-side `OptionDecl.admit` handoff 自体も actual compare には上げず、predicate fragment boundary が見えるまで docs-only deferred に留める
- stage 3 later branch の request-local clause spillover first tranche として
  - `request-local require clause is outside stage 3 admit-slot first tranche`
  - `request-local ensure clause is outside stage 3 admit-slot first tranche`
  の 2 件を substring compare で見る
- stage 3 later branch の次段 sequencing では、request head + clause attachment multiline shape より先に predicate fragment boundary の reopen 条件を docs-only で比較する
- current first choice は、declaration-side `admit` と request-local `require` / `ensure` に shared floor を与える isolated predicate fragment helper を切ることであり、program parser の accepted cluster はまだ広げない
- current actualization として、`crates/mir-ast/tests/support/current_l2_stage3_predicate_fragment_spike_support.rs` と `crates/mir-ast/tests/current_l2_stage3_predicate_fragment_spike.rs` で `e2` / `e3` / `e10` / `e11` anchor の predicate subset compare を通す first tranche までは進めてよい
- current sequencing judgment としては、predicate fragment helper の malformed-source pair を先に actualize するより、request head + clause attachment multiline shape の docs-only comparison を先に開くのが自然である
- current comparison judgment としては、multiline attachment で shared floor にするのは clause suite 全体でも generic continuation でもなく、declaration-side `admit:` と request-local `require:` / `ensure:` に共通する **single attachment frame**、すなわち `<clause-head>:` + 直下 1 段深い predicate block だけに留めるのが最小である
- current actualization として、`crates/mir-ast/tests/support/current_l2_stage3_multiline_attachment_spike_support.rs` と `crates/mir-ast/tests/current_l2_stage3_multiline_attachment_spike.rs` で declaration-side `admit:` と request-local `require:` / `ensure:` の multiline block extraction を helper-local / test-only source carrier compare まで actualize してよい
  - clause header search は head の immediate child attachment line に限る
  - blank line は helper-local で fail-closed に reject する
- current next-step sequencing としては、multiline attachment malformed-source pair をもう 1 段増やすより、request-local `require:` / `ensure:` の sibling clause suite structural floor を先に docs-only で比較するのが自然である
- current comparison judgment としては、その structural floor は `perform` owner の fixed two-slot suite に留めるのが最小である
  - `require` / `ensure` は sibling attachment line
  - current preferred ordering は `require` の後に `ensure`
  - `require` / `ensure` は各 at-most-one
  - suite termination は dedent / 次 statement head
- current next actualization judgment としては、その fixed two-slot suite floor を helper-local / test-only actual evidence に上げるとき、clause presence summary だけで止めず、`require_fragment_text` / `ensure_fragment_text` を持つ suite bridge を first tranche にするのが最小である
  - compare は existing isolated predicate fragment helper を使う
  - full request node compare や public parser API は still later stage に残す
- current actualization として、`crates/mir-ast/tests/support/current_l2_stage3_request_clause_suite_spike_support.rs` と `crates/mir-ast/tests/current_l2_stage3_request_clause_suite_spike.rs` で fixed two-slot suite bridge first tranche を helper-local / test-only actual evidence として通してよい
  - helper output は `require_fragment_text` / `ensure_fragment_text` の two-slot carrier に留める
  - single-line clause payload と multiline clause payload の両方を extracted fragment text として扱ってよい
  - helper-local structural fail-closed として
    - `require` after `ensure`
    - duplicate `require`
    - clause-between blank line
    を first tranche に入れてよい
- current next-step sequencing としては、suite bridge first tranche の直後に fixture-side full request contract compare を開くより、helper-local malformed/source family extension を先に docs-only で比較するのが自然である
- current malformed/source pair comparison としては、suite bridge family の first pair は `duplicate ensure` と unsupported direct child line を先に actualize するのが最小である
  - `missing multiline ensure block` は still later に残してよい
- current actualization としては、その first pair は `crates/mir-ast/tests/current_l2_stage3_request_clause_suite_spike.rs` の focused smoke として surfaced 済みであり、helper code widening なしに hidden fail-closed path を evidence 化してよい
- current next-step sequencing としては、その first pair actualization の後に fixture-side full request contract compare を開くより、`missing multiline ensure block` family を helper-local / test-only actual evidence として surfaced する方が自然である
- current actualization としては、その `missing multiline ensure block` hidden path も `crates/mir-ast/tests/current_l2_stage3_request_clause_suite_spike.rs` の focused smoke として surfaced 済みであり、suite bridge family の existing hidden malformed path を narrow に閉じる line まで current repo で通してよい
- current reopen judgment としては、その後に remaining suite malformed wording family を suite helper 側でまだ追うより、fixture-side full request contract compare を narrow に reopen してよい
  - ただし reopen は request head parse を still later に残し、source-side helper output を `Stage3RequestClauseSuite { require_fragment_text, ensure_fragment_text }` の two-slot carrier に留め、fixture-side compare は `contract.require` / `contract.ensure` subset だけに限定する
- current first-cut judgment としては、その reopen 後の next step は ad-hoc per-slot compare のままにするより、fixture-side expected shape を `Stage3RequestContractSubset` 相当の dedicated helper-local carrier に切るのが自然である
  - request head kind / op / target / chain_ref は still later に残す
  - source-side helper output は still `Stage3RequestClauseSuite` に留める
- helper-local malformed-source smoke として
  - `missing edge-local lineage metadata`
  - `option-local admit is outside stage 1 accepted cluster`
  の 2 件を substring compare で見る

### current non-goal

- public parser API
- final parser grammar
- parser-side span / token row
- typed parser error carrier
- detached validation loop との直接統合
- fixture-side `OptionDecl.admit` への direct lowering
- `PerformVia` / request-local `require` / `ensure`
- fixture-side `OptionDecl.admit` canonical surface reconstruction

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

## dedicated try/rollback AST structural helper first tranche の current state

current repo では、`TryFallback` / `AtomicCut` dedicated AST structural helper の first tranche が helper-local family として actualize 済みである。

- fixture-side additive fields
  - `expected_static.checked_try_rollback_structural_verdict`
  - `expected_static.checked_try_rollback_structural_findings`
- current helper-local verdict
  - `no_findings`
  - `findings_present`
- current helper-local finding rows
  - `TryFallback` / `missing_fallback_body`
  - `AtomicCut` / `disallowed_fallback_placement`
- current static malformed corpus
  - `e23-malformed-try-fallback-missing-fallback-body`
  - `e24-malformed-atomic-cut-fallback-placement`
- current loop insertion
  - static gate artifact loop の family-specific smoke
  - `scripts/current_l2_detached_loop.py smoke-try-rollback-structural-checker`

ただしこれは helper-local first tranche に留まる。
current state でも次はまだ actualize しない。

- shared detached carrier
- bundle-first runtime artifact mirror
- generic structural checker family
- public checker API
- `must_explain` の machine-check 昇格

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
  - `smoke-try-rollback-structural-checker` により、1 fixture の static gate artifact を保存し、dedicated try/rollback structural helper first tranche をその artifact に対して回せる
  - current actual cut でも、dedicated `TryFallback` / `AtomicCut` AST structural helper first tranche は bundle-first runtime path ではなく、static gate artifact を emit して helper-local compare を回す dedicated smoke family に留める
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
- `scripts/current_l2_try_rollback_structural_checker.py`
  - `TryFallback` / `AtomicCut` dedicated AST structural helper first tranche の helper-local compare
  - fixture-side `checked_try_rollback_structural_verdict` / `checked_try_rollback_structural_findings` と static gate artifact `checker_core.static_verdict` を照合する
  - `missing_fallback_body` と `disallowed_fallback_placement` だけを current first-tranche finding family として扱う
  - current phase では saved artifact path を直接 compare できるので、shared detached carrier が無くても narrow artifact recheck を回せる
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
