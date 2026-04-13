# plan/09 — helper stack と責務マップ

## 目的

current L2 parser-free PoC では、helper layer が増えるほど docs / tests / code の mirror drift が起きやすい。
ここでは helper ごとの public behavior、thin delegation、tests の literal expectation、docs mirror の境界を揃える。

## helper stack の中心関数

特に重要なのは次の stack である。

```text
run_directory_named_profile
  -> run_directory_profiled
    -> select_bundles_from_request
      -> batch_summary_from_discovery
        -> run_bundle
```

この前段に `discover_bundles_in_directory` があり、後段に `FixtureHostStub::run_fixture` と `run_to_completion` がある。

## layer ごとの public behavior / thin delegation

| helper | public behavior | thin delegation |
|---|---|---|
| `run_bundle` | 1 bundle の static / runtime / trace / host-plan coverage を照合する | oracle 実行そのものは `FixtureHostStub::run_fixture` へ委譲 |
| `batch_summary_from_discovery` | bundle 群を集計し passed / failed / coverage failure を返す | bundle 単位の実行・比較は `run_bundle` に委譲 |
| `select_bundles_from_request` | scope と single-fixture selector を逐次合成して selected discovery を返す | runtime/static classification 自体は bundle / batch 側に委譲 |
| `run_directory_profiled` | `profile_name` と selected batch summary を返す | discovery、selection、execution 自体は下位 helper を再利用 |
| `run_directory_named_profile` | alias 名を受けて named profile 実行を行い、unknown alias failure と `resolved_request` 付き summary を返す | alias 一覧列挙と alias→request 解決の source of truth は `ProfileCatalog` に委譲し、selection-shape coverage は `run_directory_profiled` へ委譲 |

## named profile catalog の責務

### public behavior

- `ProfileCatalog::aliases()`
- `ProfileCatalog::resolve()`
- unknown alias failure
- `run_directory_named_profile()` による alias→profiled execution の thin wrapper

### 持たない責務

- bundle discovery
- runtime/static-only classification
- selected bundle counts の一次計算
- fixture suffix の直接判定

## tests の責務分担

### internal tests (`crates/mir-semantics/src/harness.rs`)

- private preset table の single source of truth を確認する
- `aliases()` と `resolve()` が同じ internal table から導かれることを見る
- public integration behavior の oracle にはしない

### integration tests (`crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`)

- bundle / batch / selection / profile / named profile の public behavior を literal expectation で確認する
- `resolved_request` は literal expected request を使い、`ProfileCatalog::resolve()` 自体を test oracle に再利用しない
- unknown alias failure を literal expectation で維持する
- selected counts / concrete fixture suffix は profile-layer tests が主責務を持つ
- named-profile tests は alias / `resolved_request` / unknown alias / thin delegation を主責務にする

## docs / tests / code の mirror 境界

| 層 | 何を正本とするか |
|---|---|
| code | hard-coded preset table、helper 実装、call chain |
| tests | public behavior coverage、literal expectation、thin delegation coverage |
| docs | helper boundary、責務分担、なぜその boundary を採るかの説明 |

### alias mirror の current 方針

- concrete alias prose は `specs/examples/13-current-l2-profile-catalog.md` に寄せる
- code 側 single source of truth は `harness.rs` の hard-coded preset table
- tests 側は alias list / `resolved_request` / unknown alias failure を machine-check として持つ
- selected counts / fixture suffix は helper 化してよいが、catalog 実装を oracle に再利用しない

## drift が起きやすい点

| drift point | なぜ起きやすいか | current 対策 |
|---|---|---|
| alias list と resolve table | alias 追加時に二重定義しやすい | internal preset table の single source of truth |
| docs の alias prose | 複数 docs に同じ alias 一覧を書きたくなる | `specs/examples/13` に寄せる |
| named profile tests | selected counts / suffix / request を全部ここで持ちたくなる | profile-layer tests と named-profile tests を分離 |
| helper stack docs | 各 layer が下位 helper を再説明しがち | public behavior / thin delegation で揃える |
| sidecar / bundle / batch boundary | discovery / classification / selection を重複実装しやすい | lower-layer responsibility を plan に固定する |

## current named profile catalog の status

- hard-coded table を維持
- aliases:
  - `smoke-runtime`
  - `smoke-static`
  - `runtime-e3`
  - `static-e4`
- machine-readable catalog asset / manifest は comparison 止まり
- externalization は future option であり current L2 採用ではない

## `must_explain` の位置

helper stack のどの layer でも `must_explain` は machine-check に上げない。
これは次を守るためである。

- formal event / metadata / narrative explanation の三層分離
- report / prose 側の human-facing obligation を helper に押し込まないこと

## detached exporter entry の責務境界

detached artifact exporter を narrow に始める comparison では、次の切り分けを current understanding とする。

- payload core
  - `RunReport` に最も近い。
- first exporter entry
  - `run_bundle` / `BundleRunReport`
- later aggregate export
  - `BatchRunSummary`

この切り分けを採る理由は、`run_bundle` が current helper stack で 1 bundle の static / runtime / trace / coverage をまとめる public behavior boundaryだからである。
interpreter / `TraceAuditSink` に直結する exporter から始めると、payload core には近くても bundle context を失いやすい。
逆に batch から始めると、selection / profile / coverage aggregation まで一度に exporter へ持ち込みやすい。

## bundle-first detached artifact の split

bundle-first exporter を採る場合、`run_bundle` 周辺で見えている field は次のように分けるのが current helper boundary と最も整合する。

- payload core
  - `RunReport` 由来の field
- `bundle_context`
  - `fixture_id`
  - `fixture_path`
  - `host_plan_path`
  - `runtime_requirement`
- detached non-core
  - `steps_executed`
  - coverage explanation
  - host-plan explanation
- aggregate-only
  - `host_plan_coverage_failure`

ここで `host_plan_coverage_failure` を bundle-first artifact 側へ入れない理由は、current code でそれが `run_bundle` 成功 payload から得られる field ではなく、`batch_summary_from_discovery` の failure classification として materialize されるためである。

## `host_plan_coverage_failure` の placement 境界

current helper stack では `host_plan_coverage_failure` を次のように読むのが最も自然である。

- current detached artifact
  - aggregate-only
- 将来 typed carrier に昇格させる最小 layer
  - bundle failure artifact
- 置かない layer
  - `RunReport` payload core
  - `bundle_context`
  - detached non-core

この切り分けを採る理由は、current code ですでに per-bundle failure bit が `BatchBundleOutcome::Failed` に現れており、成功 payload や bundle identity と混ぜるより failure artifact として独立させる方が責務境界を保ちやすいためである。

将来 bundle failure artifact 側へ typed carrier を足すとしても、最小 shape は `failure_kind` discriminator だけに留める。

- `bundle_context` は別 section のまま保つ
- detached non-core の short coverage note は typed core に混ぜない
- success artifact には同名 field を持ち込まない

その次の aggregate connection でも、`batch_summary_from_discovery` と `BatchRunSummary` は coarse summary の責務を維持する。
したがって aggregate 側に typed 集約を足すとしても、最小は `failure_kind` ごとの histogram / kind count であり、bundle failure artifact の summary を薄く再掲する方向は採らない。

さらに current helper stack の naming family に合わせるなら、aggregate field 名の最小候補は `bundle_failure_kind_counts` である。

- `bundle_` prefix は `BatchRunSummary` の `bundle_failures` / `bundle_reports` とそろう
- `failure_kind` は bundle failure artifact 側の `failure.failure_kind` と 1 対 1 で接続できる
- `counts` は aggregate が coarse summary であることを保ち、histogram row の意味をそのまま表せる

docs-only migration cut としては、current `host_plan_coverage_failures` list と `BatchBundleOutcome::Failed.host_plan_coverage_failure` bool をただちに削らず、aggregate 側へ `bundle_failure_kind_counts` を additive に併存させるのが最小である。
この migration は host-stub wording、batch classifier、summary test の current anchor を壊さずに typed aggregate を差し込むための narrow cut であり、恒久の置換 timing は OPEN に残す。

## detached export loop helper の責務

detached exporter consolidation sprint の current understanding では、PoC loop を回しやすくするための non-production helper を次のように置く。

- `crates/mir-semantics/examples/current_l2_emit_detached_bundle.rs`
  - bundle-first exporter の operational aid
  - `run_bundle` / `BundleRunReport` の public behavior を再利用する
  - artifact transform 本体は `crates/mir-semantics/examples/support/current_l2_detached_bundle_support.rs` へ委譲し、example 内 private code ではなく repo 内 callable boundary として保つ
  - helper stack 本体の public API を増やさない
- `crates/mir-semantics/examples/current_l2_emit_detached_aggregate.rs`
  - aggregate exporter の operational aid
  - `run_directory` / `BatchRunSummary` の public behavior を再利用する
  - `bundle_failure_kind_counts` を migrated kind only の partial histogram として non-production aggregate artifact に narrow に落とす
  - artifact transform 本体は `crates/mir-semantics/examples/support/current_l2_detached_aggregate_support.rs` へ委譲し、example 内 private code ではなく repo 内 callable boundary として保つ
  - helper stack 本体の public API を増やさない
- `crates/mir-semantics/examples/current_l2_emit_static_gate.rs`
  - static gate artifact の operational aid
  - `load_fixture_from_path` / `static_gate_detailed` の public behavior を再利用する
  - runtime artifact と統合せず、`fixture_context` / `checker_core` だけを持つ static-only helper cut に留める
  - helper stack 本体の public API を増やさない
- `crates/mir-semantics/examples/support/current_l2_detached_aggregate_support.rs`
  - `BatchRunSummary -> detached aggregate artifact` の pure transform と carrier struct を持つ shared support helper
  - example / test からだけ読む non-production module であり、`lib.rs` / `harness.rs` の public API には入れない
- `crates/mir-semantics/examples/support/current_l2_detached_bundle_support.rs`
  - `FixtureBundle + BundleRunReport -> detached bundle artifact` の pure transform と carrier struct を持つ shared support helper
  - example / test からだけ読む non-production module であり、`lib.rs` / `harness.rs` の public API には入れない
- `crates/mir-semantics/examples/support/current_l2_static_gate_support.rs`
  - `CurrentL2Fixture + StaticGateResult -> static gate artifact` の pure transform と carrier struct を持つ shared support helper
  - example / test からだけ読む non-production module であり、`lib.rs` / `harness.rs` の public API には入れない
- `crates/mir-semantics/examples/support/current_l2_formal_hook_support.rs`
  - `DetachedStaticGateArtifact` / `DetachedBundleArtifact -> tool-neutral formal hook artifact` の pure transform と carrier struct を持つ shared support helper
  - theorem-line existing cut と同じ `subject_kind + subject_ref + contract_rows(obligation_kind + typed symbolic evidence_refs)` を維持し、source artifact schema/kind mismatch は fail-closed に止める
  - example / test からだけ読む non-production module であり、`lib.rs` / `harness.rs` の public API には入れない
- `crates/mir-semantics/examples/support/current_l2_proof_notebook_review_unit_support.rs`
  - `ToolNeutralFormalHookArtifact -> ProofNotebookReviewUnitArtifact` の pure transform と carrier struct を持つ theorem-side shared support helper
  - current cut では `subject_kind + subject_ref + row(obligation_kind + evidence_refs + goal_text) + checklist` の row-local review unit に留め、supported pair は runtime 1 件 + static 2 件だけを受け付ける
  - input formal hook に複数 contract row があっても、current helper は bridge sketch を作らず row-local review unit の list へ分解する
  - bridge sketch / compare-bless metadata / proof assistant adapter / model-check side は持ち込まず、schema/kind mismatch や unsupported pair は fail-closed に止める
  - example / test からだけ読む non-production module であり、`lib.rs` / `harness.rs` の public API には入れない
- `crates/mir-semantics/examples/current_l2_emit_proof_notebook_review_unit.rs`
  - theorem-first concrete tool pilot の thin CLI emitter
  - formal-hook JSON を読んで review-unit JSON を出すだけに留め、public theorem verifier API や retained artifact bless/update には widen しない
- `crates/mir-ast/src/current_l2.rs`
  - Phase 6 front-half parser first tranche の non-production carrier
  - stage 1 option/chain surface と stage 2 try/fallback structural surface だけを持つ
  - final parser API ではなく、checker/runtime first tranche へ渡す narrow code anchor に留める
- `crates/mir-semantics/src/lib.rs`
  - Phase 6 front-half checker/runtime first tranche の semantic entry anchor
  - `static_gate_program_detailed`、`DirectStyleEvaluator::from_program`、`run_program_to_completion` を current L2 `Program` 直受けの narrow surface として持つ
  - `CurrentL2Fixture` wrapper や public exporter API を置き換えず、parser-free baseline を壊さない
- `crates/mir-semantics/src/harness.rs`
  - Phase 6 front-half checker/runtime first tranche の host runner anchor
  - `FixtureHostStub::run_program` で host plan validation / oracle coverage check を維持したまま semantic `Program` 直実行 path を支える
  - bundle / batch / selection helper stack を public runtime API に widen しない
- `crates/mir-runtime/src/current_l2.rs`
  - Phase 6 front-half checker/runtime first tranche の non-production thin orchestrator
  - semantic `Program`、`FixtureHostPlan`、optional parser bridge input を受け、stage 1 reconnect summary、stage 2 try/rollback structural summary、static gate report、runtime report を束ねる
  - parser bridge input と semantic `Program` の mismatch は fail-closed に止め、actual parser-to-`Program` lowering first cut では `lower_current_l2_fixed_source_text` を helper-local bridge として追加してよい
  - syntax-backed sample runner first cut では `run_current_l2_source_sample` と `resolve_current_l2_source_sample_path` を helper-local wrapper として追加してよく、sample argument は explicit path / stem shorthand、host plan は explicit input に留めてよい
  - source lowerer の accepted cluster は place / option / chain / perform / single-line clause / single try / `atomic_cut` / inline admit に留め、multiline clause suite、second try、final public runtime API は still later に残す
  - runner report shape は `sample_id + sample_path + lowered + CurrentL2RuntimeSkeletonReport` に留める
  - reached stage inventory は current package では docs-first + test-only ratchet に置き、runner 本体の public shape には still 混ぜない
  - bless policy / final public CLI は still later に残す
- `scripts/current_l2_source_sample_regression.py`
  - Phase 6 source-sample authoring / bless policy current cut の repo-local helper
  - current fixed-subset first cluster inventory と first authored trio regression bundleだけを扱う
  - public runner CLI、retained artifact bless/update、authored row widen timing は扱わない
- `crates/mir-semantics/examples/current_l2_emit_formal_hook.rs`
  - Phase 6 front-half compile-ready verification / formal hook の thin CLI emitter
  - detached static gate artifact または detached bundle artifact を読み、tool-neutral formal hook artifact を JSON として出す
  - theorem/model-check concrete tool binding や public exporter API には widen しない
- `crates/mir-ast/tests/support/current_l2_stage1_parser_spike_support.rs`
  - stage 1 parser first tranche の fixture compare / summary support helper
  - parser 本体は `mir_ast::current_l2` を使い、fixture-side subset compare 用 lowering bridge と reconnect summary だけを担う
  - current widening は `Stage1ReconnectClusters` 3-bool summary contract を保ったまま `e18` / `e20` を focused test へ足す line に留め、helper 自体の contract widening は行わない
  - `mir-ast` test からだけ読む non-production module であり、fixture loader / summary helper を超えて public parser API には入れない
- `crates/mir-ast/tests/support/current_l2_stage2_try_rollback_spike_support.rs`
  - stage 2 parser first tranche の structural summary support helper
  - parser 本体は `mir_ast::current_l2` を使い、`checked_try_rollback_structural_*` と同じ helper-local summary carrier だけを担う
  - `mir-ast` test からだけ読む non-production module であり、summary helper を超えて public parser API には入れない
- `crates/mir-ast/tests/support/current_l2_stage3_admit_slot_spike_support.rs`
  - stage 3 admit-slot branch の private support helper
  - inline text から declaration-side `admit` attached slot を含む option / chain subset だけを parse し、structural subset compare と `decl_admit_slot.surface_text` retention smoke を支える
  - `mir-ast` test からだけ読む non-production module であり、public parser API や `mir-ast/src/lib.rs` には入れない
- current parser subset freeze では、stage 1 / stage 2 accepted floor は `crates/mir-ast/src/current_l2.rs` と stage 1 / stage 2 parser spike tests に actualize 済みであり、stage 3 support helper は retained-later floor evidence として扱う
- current parser-to-checker reconnect freeze では、stage 1 support helper は `Stage1ReconnectClusters` summary floor、stage 2 support helper は `checked_try_rollback_structural_*` floor をそれぞれ source-backed bridge evidence として扱う。Phase 6 checker/runtime first tranche では、この bridge evidence を `mir-runtime::current_l2` の optional parser bridge input と consistency guard へ narrow actualize した。
- stage 3 request / predicate reconnect helper line、`e19` direct target mismatch redesign、`E21` / `E22` runtime contrast は helper stack の current bridge contract には入れず、later reopen line として扱う
- `scripts/current_l2_diff_detached_artifacts.py`
  - detached artifact の payload core だけを比較する repo-level helper
  - `must_explain` を比較対象に上げない
  - `bundle_context` / `detached_noncore` は reference-only として読む
- `scripts/current_l2_diff_detached_aggregates.py`
  - aggregate artifact の `summary_core` だけを比較する repo-level helper
  - `bundle_failure_kind_counts_scope = "migrated-kinds-only"` を core compare に残す
  - `aggregate_context` / `detached_noncore` は reference-only として読む
- `scripts/current_l2_diff_static_gate_artifacts.py`
  - static gate artifact の `checker_core` だけを比較する repo-level helper
  - `fixture_context` と helper-local `detached_noncore.reason_codes` は reference-only として読む
- `scripts/current_l2_detached_loop.py`
  - bundle-first emitter、aggregate emitter、bundle diff helper、aggregate diff helper を current validation loop 向けに束ねる薄い wrapper
  - compile-ready checkpoint では `smoke-formal-hook-static` / `smoke-formal-hook-runtime` も束ね、formal hook emitted path の smoke gate を持つ
  - `target/current-l2-detached/` を current non-production default candidate として扱う
  - explicit path compare、fixture-to-artifact compare、aggregate summary export、run-label aggregate compare を最小で支える
  - `smoke-fixture` subcommand では、1 fixture の bundle emit、optional reference compare、single-fixture aggregate smoke を 1 command で支える
  - current first tranche では fixture 引数の stem shorthand 解決と missing fixture の fail-fast を担ってよい
  - current second tranche では `compare-fixture-aggregates` により、single-fixture aggregate 同士の direct compare convenience を担ってよい
  - current third tranche では bundle / aggregate / static gate diff helper の reference-only section を whole-section blob でなく shallow per-field summary に崩し、longer compare triage を短くしてよい
  - `reference update / bless` は compare helper と同じ layerへまだ入れず、path policy / retention policy と接続する later candidate に残してよい
  - compare helper の exit code `1` は informational difference として扱い、そのことを short note で補足してよい

## parser boundary 側の current splice point

- current parser first tranche は `mir-ast/src/current_l2.rs` に narrow に actualize 済みである
- ただし actualized したのは stage 1 / stage 2 carrier だけであり、helper stack の public chain
  - fixture schema
  - parser-free interpreter
  - detached validation loop
  を壊さない
  - stage 3 admit-slot branch でも同じ cut を維持し、fixture-side `admit` node や request cluster を first-tranche carrier へ持ち込まない
  - `smoke-try-rollback-locality` subcommand では、`e22` mismatch 側と `e21` frontier 側を representative contrast pair として既定パス / label 付きでまとめて回す
  - `smoke-static-gate` subcommand では、1 fixture の static gate artifact emit と optional reference compare を 1 command で支える
  - `smoke-try-rollback-structural-checker` subcommand では、1 fixture の static gate artifact emit と dedicated try/rollback structural helper first tranche compare を 1 command で支える
  - `suggest-checked-reasons` subcommand では、1 fixture の static gate artifact を emit した後に display-only assist を呼び、fixture-side `expected_static.checked_reasons` 候補を表示する
  - `suggest-reason-codes` subcommand では、1 fixture の static gate artifact を emit した後に display-only assist を呼び、future typed carrier 候補 row を reference-only で表示する
  - current actual cut でも、dedicated `TryFallback` / `AtomicCut` AST structural helper first tranche は `smoke-fixture` や `smoke-try-rollback-locality` ではなく、static gate artifact emit のあとに helper-local compare を回す dedicated smoke family に留める
  - compare helper の exit code `1` は difference found として informational に許容し、emitter / helper failure だけを non-zero で返す
- `scripts/current_l2_scaffold_fixture.py`
  - fixture authoring の boilerplate だけを current validation loop の手前で補助する
  - runtime / static-only の scaffold と empty `.host-plan.json` sidecar 骨格だけを作る
  - expectation completion、review、detached export 自体は下位の authoring / loop に残す
- `scripts/current_l2_checked_reasons_assist.py`
  - static gate artifact の `checker_core.reasons` を読んで、fixture-side `expected_static.checked_reasons` の copyable suggestion を display-only で返す
  - fixture JSON の自動更新や `checked_reasons = []` の一括補完は行わない
- `scripts/current_l2_reason_codes_assist.py`
  - static gate artifact の helper-local / reference-only `detached_noncore.reason_codes` を読んで、future typed carrier 候補 row を display-only で返す
  - current first typed tranche では fixture-side `expected_static.checked_reason_codes` の有無と actual suggestion の一致も display-only で確認してよい
  - unsupported legacy fixture-side typed field を見つけたら fail-closed に止まる
  - fixture JSON の自動更新や typed field の仮挿入は行わない
- `scripts/current_l2_reason_code_readiness.py`
  - static-only fixture corpus を横断し、`checked_reasons` adoption と `detached_noncore.reason_codes` suggestion availability を batch で display-only 要約する
  - current stable cluster tranche の `checked_reason_codes` adoption 数も同じ scan に載せてよい
  - coexistence follow-up 用に、stable coexistence anchor 数、missing `checked_reasons` 数、typed mismatch 数も同じ scan に載せてよい
  - first checker cut の regression baseline として、stable kind を checker cluster に roll-up した coverage count も同じ scan に載せてよい
  - stable cluster と duplicate cluster の current split を authoring tranche 単位で観察するが、machine-check core や detached aggregate には上げない
- `scripts/current_l2_same_lineage_checker.py`
  - same-lineage static evidence floor に限った first checker spike
  - fixture schema や detached artifact schema を増やさず、既存 `checked_reason_codes` と static gate artifact `reason_codes` を読む helper-local compare に留める
- `scripts/current_l2_missing_option_checker.py`
  - missing-option structure floor に限った second checker spike
  - fixture schema や detached artifact schema を増やさず、既存 `checked_reason_codes` と static gate artifact `reason_codes` を読む helper-local compare に留める
- `scripts/current_l2_capability_checker.py`
  - capability strengthening floor に限った third checker spike
  - fixture schema や detached artifact schema を増やさず、既存 `checked_reason_codes` と static gate artifact `reason_codes` を読む helper-local compare に留める
- `scripts/current_l2_try_rollback_structural_checker.py`
  - `TryFallback` / `AtomicCut` dedicated AST structural helper first tranche の helper-local compare
  - `checked_try_rollback_structural_verdict` / `checked_try_rollback_structural_findings` と static gate artifact `checker_core.static_verdict` を narrow に照合する
  - helper-local checker 自体が saved artifact path を直接受けるので、current first tranche の narrow artifact recheck は shared detached carrier へ上げずに支えられる
  - shared family support helper や public checker API には上げない
- `scripts/current_l2_family_checker_support.py`
  - same-lineage / missing-option / capability の 3 checker spike が共有する parser / row filter / status / stdout contract をまとめる
  - family facade script は残し、generic checker-side shared CLI や public checker API にはしない

`scripts/current_l2_detached_loop.py` では、`scan-reason-code-readiness` subcommand を追加してよい。

- static-only fixture だけに static gate artifact emit を行う
- runtime fixture は skipped count にだけ入れる
- 上記 readiness helper を呼び、1 fixture assist と corpus 横断 scan を同じ wrapper family に収める
- `smoke-same-lineage-checker` を追加し、static gate artifact emit と same-lineage first checker spike を同じ wrapper family に収めてよい
- `smoke-missing-option-checker` を追加し、static gate artifact emit と missing-option second checker spike を同じ wrapper family に収めてよい
- `smoke-capability-checker` を追加し、static gate artifact emit と capability third checker spike を同じ wrapper family に収めてよい
- `smoke-try-rollback-structural-checker` を current actual command surface として追加してよく、dedicated `TryFallback` / `AtomicCut` AST structural helper first tranche はこれらと同じ static-gate-side wrapper family に置き、bundle-first runtime path や generic checker-side shared entry へは混ぜない
- ただし fixture JSON 自動更新や typed carrier actualization は行わない
- family-specific smoke command 名は維持し、shared support helper 導入だけで wrapper public surface を置き換えない

これらを `harness.rs` 本体へ入れない理由は次の通りである。

- non-production helper を current public helper stack と混同させないため
- actual exporter API を既成事実化しないため
- diff helper を test oracle や host harness implementation と切り離すため

`tests/support` に置かない理由は、test-only support に寄せると repo 外 artifact compare の補助として再利用しにくく、PoC loop の operational aid としての性格が見えにくくなるためである。

さらに current helper boundary では、artifact 保存先 / path policy を helper stack 本体へ埋め込まない。
現在の最小 cut は次である。

- docs-only の current non-production default candidate
  - `target/current-l2-detached/`
- tiny wrapper がその candidate を使うことは許す
- `run_bundle` / `BatchRunSummary` / `harness.rs` 本体には final storage policy を持ち込まない

aggregate export も同じである。

- `BatchRunSummary` は coarse summary の責務を維持する
- `bundle_failure_kind_counts` は docs-only の additive typed field 候補として扱う
- current non-production aggregate emitter sketch は許容する
- ただし actual aggregate exporter API は `harness.rs` の public behavior と切り分けたまま OPEN に残す

## current public operational surface inventory

current repo の public operational surface inventory では、少なくとも 3 bucket を分けて読むのが自然である。

### 1. already-public parser-free helper stack

- `mir-semantics` crate の parser-free public behavior であり、
  - `run_bundle`
  - `discover_bundles_in_directory`
  - `select_bundles_from_request`
  - `run_directory_profiled`
  - `run_directory_named_profile`
  を中心とする bundle / discovery / selection / profile stack を指す。
- これは current repo で already-public behavior と読んでよく、Phase 6 current tranche の visibility と混ぜない。

### 2. crate-public but non-production compile-ready tranche

- `mir_ast::current_l2`
- `mir_runtime::current_l2`
- `static_gate_program_detailed`
- `DirectStyleEvaluator::from_program`
- `run_program_to_completion`
- `FixtureHostStub::run_program`

これらは crate から見えていても、current docs では non-production thin tranche と読む。
Rust の `pub` visibility だけで final public operational contract に昇格したと解釈しない。

### 3. repo-local helper / example emitter surface

- `crates/mir-semantics/examples/*`
- `crates/mir-semantics/examples/support/*`
- `scripts/current_l2_detached_loop.py`
- `scripts/current_l2_source_sample_regression.py`
- family checker / diff helper / assist helper 群

これらは repo-local operational aid であり、public crate contract とは別 bucket に残す。

## inventory guard

- already-public parser-free stack を current tranche の later promotion pressureで巻き戻さない。
- crate-public symbol をそのまま final public API と誤読しない。
- example/support helper と Python orchestration helper を public crate namespace へ hidden promotion しない。
- final public parser / checker / runtime API、public runner / exporter CLI、public theorem / model-check / checker migration は still later に残す。

## current public operational surface actualization gate

public operational surface inventory の次段 current gate では、
**compile-ready tranche 全体を一括で public 候補にせず、`run_current_l2_source_sample` を first docs-only public-pressure candidate に narrow に置く**
のが自然である。

### stable public bucket

- parser-free helper stack
  - `run_bundle`
  - `discover_bundles_in_directory`
  - `select_bundles_from_request`
  - `run_directory_profiled`
  - `run_directory_named_profile`

### first candidate

- `mir_runtime::current_l2::run_current_l2_source_sample`

これは parser carrier、program-level semantic entry、source-backed sample execution を end-to-end で跨ぐ narrow candidate である。

### tranche-internal support

- `mir_runtime::current_l2::run_current_l2_runtime_skeleton`
- `mir_runtime::current_l2::lower_current_l2_fixed_source_text`
- `mir_ast::current_l2`
- `static_gate_program_detailed`
- `DirectStyleEvaluator::from_program`
- `run_program_to_completion`
- `FixtureHostStub::run_program`

これらは current gate では support / carrier bucket に留める。

### excluded bucket

- `mir_runtime::current_l2::resolve_current_l2_source_sample_path`
- hard-coded accepted sample set
- `crates/mir-semantics/examples/*`
- `crates/mir-semantics/examples/support/*`
- `scripts/current_l2_source_sample_regression.py`
- `scripts/current_l2_detached_loop.py`
- other repo-local assist / checker / diff helpers

### gate guard

- repo layout と accepted-set hard-coding を final public contract に混ぜない。
- explicit `FixtureHostPlan` coupling を current candidate のまま固定しない。
- partial parser surface を crate/module 単位で promotion しない。
- `pub` visibility を final public contract と読まない。

## current public operational later-gate ordering

public operational surface actualization gate の次段 current reading では、
**library-side final public contract を first later gate、public operational CLI を second later gate に置く**
のが自然である。

この later ordering は維持するが、current roadmap では **その前段に sample-visible theorem/model-check milestone を置き、その後に docs-first の host-facing I/O / adapter boundary を別 gate として切る** のが自然である。ここでいう `host-facing port` は working label であり、final terminology は OPEN に残す。

### first later gate

- final public parser / checker / runtime API

current candidate が `run_current_l2_source_sample` という library-side entry であるため、その先の first later gate も library-side contract と読むのが最小である。

### second later gate

- public operational CLI

CLI は library-side final contract の上に載る operational wrapper として別 gate に残す。

### later docs-first host-facing boundary

- capability-scoped I/O / port boundary（working label）
- visualizer / host substrate adapter
- FFI / engine adapter

これは current public operational surface actualization gate そのものではなく、library-before-CLI later orderingを保ったまま、host-facing integration を `stdin/stdout` privileged 化せずに整理する separate gate として扱う。

### kept-out pressure

- `mir_runtime::current_l2::resolve_current_l2_source_sample_path`
- hard-coded accepted sample set
- `crates/mir-semantics/examples/*`
- `crates/mir-semantics/examples/support/*`
- `scripts/current_l2_source_sample_regression.py`
- `scripts/current_l2_detached_loop.py`

これらは repo layout / repo-local helper surface を持つため、current final public contract の外に残す。

## この先の update 指針

helper layer が変わったら、少なくとも次のどれを更新すべきかを見る。

- `plan/07-parser-free-poc-stack.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `specs/examples/09..13`
- `Documentation.md`

更新不要なら、その task の report に `plan/ 更新不要` を明記する。
