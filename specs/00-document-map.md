# 00 — 文書マップ

この文書は、主要な関心事がそれぞれどこに書かれているかを読者または agent に示す。

## planning document

- `plan/`
  - `specs/` と `docs/reports/` と code anchor を横断して、現況、roadmap、helper stack、fixture catalog、open problems、maintenance rule を長期参照しやすく整理した人間向け repository memory である。
  - 規範判断の正本ではない。意味論や decision の正本は `specs/` に残る。
  - current repo の現在地や PoC 検証基盤の call chain を素早く掴みたい場合は `plan/00-index.md` から読む。

## 規範的な読書順

1. `01-charter-and-decision-levels.md`
2. `02-system-overview.md`
3. `03-layer-model.md`
4. `09-invariants-and-constraints.md`
5. 1 つ以上の subsystem 文書
   - `04-mir-core.md`
   - `05-mirrorea-fabric.md`
   - `06-prismcascade-positioning.md`
   - `07-typed-effects-wiring-platform.md`
   - `08-cross-system-relations.md`
6. `10-open-questions.md`
7. `11-roadmap-and-workstreams.md`
8. `12-decision-register.md`

## 例示文書

- `specs/examples/00-representative-mir-programs.md`
  - current L2 の representative Mir programs をまとめた例示文書。
  - parser / interpreter を固定するものではなく、コード片ごとの static 判定、runtime outcome、最小 trace 説明を揃えるための補助正本である。
  - `specs/04-mir-core.md`、`specs/10-open-questions.md`、`specs/12-decision-register.md` を読んだ後に参照すると、現時点で何が自然に書けるかを追いやすい。
- `specs/examples/01-current-l2-surface-syntax-candidates.md`
  - representative examples で使う `perform`、option chain 参照、`try` / `fallback`、`require` / `ensure` clause、statement separator / block nesting の current L2 候補書式をまとめた補助文書。
  - final parser syntax や reserved keyword を固定するものではなく、examples 用の安定した companion notation を示す。
- `specs/examples/02-current-l2-ast-fixture-schema.md`
  - representative examples を parser なしで machine-readable に扱うための current L2 AST fixture schema をまとめた補助文書。
  - syntax の punctuation を固定せず、意味側の構造と expected static / runtime / trace-audit を fixture 化する。
- `specs/examples/03-current-l2-evaluation-state-schema.md`
  - parser なし最小 interpreter に必要な current L2 evaluation state schema をまとめた補助文書。
  - AST fixture から runtime state へ渡す最小 carrier と、E1 / E2 / E3 variant / E6 を動かすのに必要な state 粒度を整理する。
- `specs/examples/04-current-l2-step-semantics.md`
  - parser なし最小 interpreter の current L2 step semantics をまとめた補助文書。
  - Program / PlaceBlock / PerformOn / PerformVia / TryFallback / AtomicCut の 1-step 規則と、E1 / E2 / E3 variant / E6 の walkthrough を整理する。
- `specs/examples/05-current-l2-oracle-api.md`
  - parser なし最小 interpreter の current L2 predicate / effect oracle API をまとめた補助文書。
  - `PerformOn` / `PerformVia` が oracle に渡す最小 input、oracle が返す最小 carrier、step semantics との接続を整理する。
- `specs/examples/06-current-l2-interpreter-skeleton.md`
  - parser なし最小 interpreter skeleton の current L2 実装境界をまとめた補助文書。
  - static gate、runtime evaluation、machine-checked expectation、future work の境界を整理する。
- `specs/examples/07-current-l2-host-stub-harness.md`
  - parser なし minimal interpreter を fixture ごとに検証する current L2 host stub / harness の補助文書。

- `specs/examples/08-current-l2-host-plan-schema.md`
  - current L2 host harness が読む machine-readable host plan schema と sidecar JSON 方針をまとめた補助文書。
  - predicate verdict、effect outcome、success-side carrier、trace expectation override を declarative に差し替える最小 test harness を整理する。
- `specs/examples/09-current-l2-bundle-loader.md`
  - current L2 fixture と `.host-plan.json` sidecar を 1 組として扱う bundle loader / bundle-level helper の補助文書。
  - static gate fixture と runtime fixture を同じ入口から扱い、machine-check と human-facing explanation obligation の境界を bundle 層で維持する。
- `specs/examples/10-current-l2-batch-runner.md`
  - current L2 fixture directory を bundle 群として一括 discovery / 実行する batch runner の補助文書。
  - runtime bundle / static-only bundle の振り分け、summary report、machine-check と human-facing explanation obligation の境界を directory 層で維持する。
- `specs/examples/11-current-l2-selection-helper.md`
  - current L2 batch runner の上で bundle 群を `runtime-only` / `static-only` / `single-fixture` に選別する helper の補助文書。
  - batch discovery rule を変えず、選別後の summary と machine-check / human-facing explanation の境界だけを整理する。
- `specs/examples/12-current-l2-selection-profiles.md`
  - current L2 selection helper の primitive mode を組み合わせ、profile 名付き summary で selected batch 実行するための最小 profile helper 境界。
- `specs/examples/13-current-l2-profile-catalog.md`
  - current L2 selection profile helper の上に small named profile catalog / preset table を薄く載せ、human-friendly alias を既存 request へ解決するための最小境界。
- `specs/examples/14-current-l2-profile-catalog-externalization.md`
  - current L2 named profile catalog を hard-coded table のまま維持するか、machine-readable catalog asset / preset manifest へ外出しするかを比較する補助文書。
  - production manifest を固定せず、PoC 実験ループの観点から current L2 でどこまで externalization する価値があるかだけを整理する。
- `specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md`
  - current L2 の fallback / `lease` reading と「外側 option の寿命延長」直感とのズレを整理し、fallback / preference chain の compact syntax candidate を比較する補助文書。
  - semantics は変えず、current L2 companion notation にどこまで compact candidate を昇格させるかだけを整理する。
- `specs/examples/16-current-l2-detached-trace-audit-artifact-schema.md`
  - current L2 parser-free PoC の trace / audit 結果を detached artifact として外へ出すとき、何を exact-compare core に残し、何を detached non-core とし、何を human-facing explanation に残すかを整理する補助文書。
  - production serialization format や richer host interface を固定せず、docs-only minimal schema だけを与える。
- `specs/examples/17-current-l2-detached-exporter-entry-comparison.md`
  - current L2 parser-free PoC で detached artifact exporter を narrow に始めるとき、`RunReport` / `BundleRunReport` / `BatchRunSummary` のどこを entry layer に取るべきかを比較する補助文書。
  - production exporter 実装を固定せず、payload core と first exporter entry boundary の切り分けだけを与える。
- `specs/examples/18-current-l2-bundle-first-detached-payload-context-split.md`
  - current L2 parser-free PoC の bundle-first detached exporter で、何を `RunReport` 由来 payload core に置き、何を `bundle_context` に置き、何を detached non-core とし、何を explanation に残すかを比較する補助文書。
  - production exporter 実装を固定せず、bundle-first artifact の docs-only payload/context split だけを与える。
- `specs/examples/19-current-l2-host-plan-coverage-failure-placement.md`
  - current L2 parser-free PoC の detached artifact で `host_plan_coverage_failure` を current では aggregate-only に残しつつ、将来 typed carrier に昇格させるならどの layer が自然かを比較する補助文書。
  - production exporter 実装や richer host interface を固定せず、placement boundary だけを与える。
- `specs/examples/20-current-l2-host-plan-coverage-failure-bundle-failure-artifact-schema.md`
  - current L2 parser-free PoC の detached artifact で、`host_plan_coverage_failure` を bundle failure artifact 側の typed carrier に昇格させるときの最小 schema を比較する補助文書。
- `specs/examples/21-current-l2-host-plan-coverage-failure-aggregate-connection.md`
  - current L2 parser-free PoC の detached artifact で、bundle failure artifact 側の `failure.failure_kind` discriminator-only schema を `BatchRunSummary` aggregate export がどこまで typed に吸うべきかを比較する補助文書。
  - production exporter 実装や richer host interface を固定せず、aggregate connection の docs-only boundary judgment だけを与える。
- `specs/examples/22-current-l2-host-plan-coverage-failure-aggregate-histogram-migration.md`
  - current L2 parser-free PoC の detached artifact で、aggregate export 側に typed histogram / kind count を入れるなら、その field 名と migration cut をどう切るのが最小かを比較する補助文書。
  - production exporter 実装や actual schema version を固定せず、aggregate naming と docs-only migration cut だけを与える。
- `specs/examples/23-current-l2-detached-export-loop-consolidation.md`
  - current L2 parser-free PoC の detached exporter chain を 1 箇所へ統合し、payload core / `bundle_context` / detached_noncore / explanation の cut、`host_plan_coverage_failure` の current state、future typed bundle failure artifact と aggregate histogram migration の current understanding を集約する補助文書。
  - production exporter API や保存先 policy を固定せず、non-production の loop attachment と next narrow step だけを整理する。
- `specs/examples/24-current-l2-detached-export-storage-and-aggregate-api.md`
  - current L2 parser-free PoC の detached validation loop で、bundle-first artifact の保存先 / path policy、compare input discovery、aggregate export への接続面、`bundle_failure_kind_counts` の additive coexistence をどう切るかを整理する補助文書。
  - production exporter API や final path policy を固定せず、non-production loop を回しやすくする最小 storage / aggregate cut だけを与える。
- `specs/examples/25-current-l2-detached-aggregate-emitter-sketch.md`
  - current L2 parser-free PoC の detached validation loop で、`run_directory` / `BatchRunSummary` 起点の aggregate emitter を non-production helper としてどこまで許すかを整理する補助文書。
  - production exporter API を固定せず、aggregate artifact の actual narrow cut と current storage / migration judgment との接続だけを与える。
- `specs/examples/26-current-l2-detached-aggregate-compare-helper.md`
  - current L2 parser-free PoC の detached validation loop で、aggregate artifact 2 本の `summary_core` をどこまで exact-compare し、`aggregate_context` / `detached_noncore` をどこまで reference-only に留めるかを整理する補助文書。
  - production compare API を固定せず、`compare-aggregates` wrapper までを current non-production convenience として与える。

## 各文書の役割

- `01-charter-and-decision-levels.md`
  - プロジェクトの意図、スコープ、decision level system。
- `02-system-overview.md`
  - スタック全体の高水準な概要。
- `03-layer-model.md`
  - 各 layer の正確な責務と境界。
- `04-mir-core.md`
  - Mir の言語コア、四本柱、意味論中心の要約。
- `05-mirrorea-fabric.md`
  - naming、routing、overlay、patching、audit、distributed control。
- `06-prismcascade-positioning.md`
  - なぜ PrismCascade が分離されるのか、その統合面は何か。
- `07-typed-effects-wiring-platform.md`
  - Typed-effect routing / container visibility platform とその役割。
- `08-cross-system-relations.md`
  - 共有前提、境界、相互運用点。
- `09-invariants-and-constraints.md`
  - 黙って破ってはならない最重要ルール。
- `10-open-questions.md`
  - 明示的な未解決事項。
- `11-roadmap-and-workstreams.md`
  - 提案されている実装 / 研究の順序。
- `12-decision-register.md`
  - 現在の主要判断と、その強度レベル。
- `specs/examples/00-representative-mir-programs.md`
  - current L2 の representative Mir program と、その static 判定 / runtime outcome / trace の最小読解。
- `specs/examples/01-current-l2-surface-syntax-candidates.md`
  - current L2 の representative examples で使う最小 surface syntax 候補。
- `specs/examples/02-current-l2-ast-fixture-schema.md`
  - current L2 の representative examples を parser なしで fixture に落とすための最小 AST schema。
- `specs/examples/03-current-l2-evaluation-state-schema.md`
  - current L2 の representative examples を parser なし最小 interpreter で読むための最小 evaluation state schema。
- `specs/examples/04-current-l2-step-semantics.md`
  - current L2 の representative examples を parser なし最小 interpreter で進めるための最小 step semantics。
- `specs/examples/05-current-l2-oracle-api.md`
  - current L2 の representative examples を parser なし最小 interpreter で動かすための最小 oracle boundary。
- `specs/examples/06-current-l2-interpreter-skeleton.md`
  - current L2 の representative examples を parser なし最小 interpreter skeleton で動かすときの最小実装境界。
- `specs/examples/07-current-l2-host-stub-harness.md`
  - current L2 の representative examples を parser なし minimal interpreter で検証しやすくする host stub / harness の最小境界。
- `specs/examples/08-current-l2-host-plan-schema.md`
  - current L2 host harness 用の machine-readable host plan schema と `.host-plan.json` sidecar asset の最小方針。
- `specs/examples/09-current-l2-bundle-loader.md`
  - current L2 fixture と sidecar asset を 1 つの bundle として load / run / verify するための最小 helper 境界。
- `specs/examples/10-current-l2-batch-runner.md`
  - current L2 fixture directory を bundle 群として一括 discovery / 実行 / 集計するための最小 batch helper 境界。
- `specs/examples/11-current-l2-selection-helper.md`
  - current L2 bundle 群を `runtime-only` / `static-only` / `single-fixture` で絞って batch 実行するための最小 selection helper 境界。
- `specs/examples/12-current-l2-selection-profiles.md`
  - current L2 selection helper の primitive mode を組み合わせ、profile 名付き summary で selected batch 実行するための最小 profile helper 境界。
- `specs/examples/13-current-l2-profile-catalog.md`
  - current L2 selection profile helper の上に thin な named alias layer を足し、human-friendly preset 名を既存 request へ解決するための最小 catalog 境界。
- `specs/examples/14-current-l2-profile-catalog-externalization.md`
  - current L2 named profile catalog を hard-coded table に留めるか、machine-readable asset として比較するための最小 companion 境界。
- `specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md`
  - current L2 の guarded option chain 読みと outer-longer-lifetime 直感のズレを説明し、compact syntax candidate の比較と暫定 companion notation 判断を与える。
- `specs/examples/16-current-l2-detached-trace-audit-artifact-schema.md`
  - current L2 parser-free PoC の trace / audit 結果を detached artifact として切り出すときの最小 schema と、exact-compare core / detached non-core / human-facing explanation の境界を与える。
- `specs/examples/17-current-l2-detached-exporter-entry-comparison.md`
  - current L2 parser-free PoC の detached artifact exporter を narrow に始めるとき、payload core と entry layer をどう切り分けるかを比較する。
- `specs/examples/18-current-l2-bundle-first-detached-payload-context-split.md`
  - current L2 parser-free PoC の bundle-first detached artifact で、payload core / bundle context / detached non-core / human-facing explanation をどう切るかを比較する。
- `specs/examples/19-current-l2-host-plan-coverage-failure-placement.md`
  - current L2 parser-free PoC の detached artifact で、`host_plan_coverage_failure` をどの layer に置くのが自然かを比較する。
- `specs/examples/20-current-l2-host-plan-coverage-failure-bundle-failure-artifact-schema.md`
  - current L2 parser-free PoC の detached artifact で、`host_plan_coverage_failure` を bundle failure artifact 側の typed carrier に昇格させる場合の最小 schema を比較する。
- `specs/examples/21-current-l2-host-plan-coverage-failure-aggregate-connection.md`
  - current L2 parser-free PoC の detached artifact で、bundle failure artifact 側の `failure.failure_kind` を `BatchRunSummary` aggregate export がどこまで typed に集約すべきかを比較する。
- `specs/examples/22-current-l2-host-plan-coverage-failure-aggregate-histogram-migration.md`
  - current L2 parser-free PoC の detached artifact で、aggregate export 側に typed histogram / kind count を入れるなら、その field 名と migration cut をどう切るかを比較する。
- `specs/examples/23-current-l2-detached-export-loop-consolidation.md`
  - current L2 parser-free PoC の detached exporter chain を 1 箇所へ統合し、bundle-first loop attachment と typed aggregate migration の current understanding を集約する。
- `specs/examples/24-current-l2-detached-export-storage-and-aggregate-api.md`
  - current L2 parser-free PoC の detached validation loop で、artifact 保存先 / path policy / aggregate export 接続の最小 cut を整理する。
- `specs/examples/25-current-l2-detached-aggregate-emitter-sketch.md`
  - current L2 parser-free PoC の detached validation loop で、aggregate emitter の actual narrow cut と `bundle_failure_kind_counts` / current list anchor coexistence を整理する。
- `specs/examples/26-current-l2-detached-aggregate-compare-helper.md`
  - current L2 parser-free PoC の detached validation loop で、aggregate compare helper の exact-compare core / reference-only split と run-label convenience wrapper を整理する。
- `specs/examples/27-current-l2-fixture-scaffold-helper.md`
  - current L2 parser-free PoC の fixture authoring で、required carrier と empty `.host-plan.json` sidecar 骨格だけを `target/` 下へ出す non-production scaffold helper の最小境界を整理する。

## レポート

- `docs/reports/` には時系列の作業ログが入る。
- report 自体は規範文書ではないが、なぜ変更が行われたかを説明する。

## 補助文書

- `specs/examples/`
  - representative program、説明用記法、例示中心の補助文書置き場。
  - parser なしで扱う representative example fixture の schema は `specs/examples/02-current-l2-ast-fixture-schema.md` に置き、machine-readable な fixture 実体は `crates/mir-ast/tests/fixtures/current-l2/` に置く。
  - parser なし最小 interpreter に必要な evaluation state schema は `specs/examples/03-current-l2-evaluation-state-schema.md` に置く。
  - parser なし最小 interpreter の 1-step semantics は `specs/examples/04-current-l2-step-semantics.md` に置く。
  - parser なし最小 interpreter の predicate / effect oracle API は `specs/examples/05-current-l2-oracle-api.md` に置く。
  - parser なし最小 interpreter skeleton の実装境界は `specs/examples/06-current-l2-interpreter-skeleton.md` に置く。
  - parser なし minimal interpreter の host stub / fixture runner harness は `specs/examples/07-current-l2-host-stub-harness.md` に置く。
  - parser なし minimal interpreter の machine-readable host plan schema と sidecar loader 方針は `specs/examples/08-current-l2-host-plan-schema.md` に置く。
  - parser なし minimal interpreter の fixture bundle loader / bundle-level helper は `specs/examples/09-current-l2-bundle-loader.md` に置く。
  - parser なし minimal interpreter の fixture directory batch runner / summary helper は `specs/examples/10-current-l2-batch-runner.md` に置く。
  - parser なし minimal interpreter の filtered batch runner / selection helper は `specs/examples/11-current-l2-selection-helper.md` に置く。
  - parser なし minimal interpreter の selection mode を組み合わせる profile helper は `specs/examples/12-current-l2-selection-profiles.md` に置く。
  - parser なし minimal interpreter の small named profile catalog / preset table は `specs/examples/13-current-l2-profile-catalog.md` に置く。
  - parser なし minimal interpreter の named profile catalog を hard-coded table に留めるか、machine-readable asset として比較する整理は `specs/examples/14-current-l2-profile-catalog-externalization.md` に置く。
  - parser なし minimal interpreter の trace / audit 結果を repo 外へ保存する detached artifact 境界の docs-only minimal schema は `specs/examples/16-current-l2-detached-trace-audit-artifact-schema.md` に置く。
  - parser なし minimal interpreter の detached artifact exporter をどの helper layer から始めるかの comparison は `specs/examples/17-current-l2-detached-exporter-entry-comparison.md` に置く。
  - parser なし minimal interpreter の bundle-first detached exporter で payload と context をどう split するかの comparison は `specs/examples/18-current-l2-bundle-first-detached-payload-context-split.md` に置く。
  - parser なし minimal interpreter の `host_plan_coverage_failure` を aggregate-only に残すか、将来 typed carrier として bundle failure artifact 側へ降ろすかの comparison は `specs/examples/19-current-l2-host-plan-coverage-failure-placement.md` に置く。
  - parser なし minimal interpreter の `host_plan_coverage_failure` を bundle failure artifact 側の typed carrier に昇格させる場合の最小 schema refinement は `specs/examples/20-current-l2-host-plan-coverage-failure-bundle-failure-artifact-schema.md` に置く。
  - parser なし minimal interpreter の bundle failure artifact 側 `failure.failure_kind` を `BatchRunSummary` aggregate export がどこまで typed に吸うべきかの comparison は `specs/examples/21-current-l2-host-plan-coverage-failure-aggregate-connection.md` に置く。
  - parser なし minimal interpreter の detached validation loop で、artifact 保存先 / path policy と aggregate export の最小 API cut をどこに置くかの整理は `specs/examples/24-current-l2-detached-export-storage-and-aggregate-api.md` に置く。
  - parser なし minimal interpreter の aggregate emitter sketch と `bundle_failure_kind_counts` / current list anchor coexistence の actual narrow cut は `specs/examples/25-current-l2-detached-aggregate-emitter-sketch.md` に置く。
  - parser なし minimal interpreter の fixture authoring で required carrier だけを scaffold する non-production helper 境界は `specs/examples/27-current-l2-fixture-scaffold-helper.md` に置く。
  - parser なし minimal interpreter の detached validation loop で 1 fixture を bundle export / optional reference compare / single-fixture aggregate smoke まで 1 command で回す helper 境界は `specs/examples/28-current-l2-detached-fixture-validation-loop-helper.md` に置く。
  - current L2 companion notation から final grammar へ進む前に、first parser cut に入れてよい semantic cluster と companion に残す cluster を narrow に棚卸しする inventory は `specs/examples/29-current-l2-first-parser-subset-inventory.md` に置く。
  - first parser cut inventory の次段として、current L2 で core checker に入れてよい local / structural judgment と external verifier 側へ残す judgment の entry criteria は `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md` に置く。
  - ここにあるコード片は parser-ready な最終 syntax を固定するものではなく、規範文書の current reading を具体例として読むための companion として扱う。

## 実装 anchor

- `crates/mir-ast/tests/fixtures/current-l2/`
  - current L2 parser-free PoC の representative fixture 実体と `.host-plan.json` sidecar を置く。
- `crates/mir-semantics/src/lib.rs`
  - parser-free minimal interpreter の entry point と evaluation 実装。
- `crates/mir-semantics/src/harness.rs`
  - host harness、host plan loader、bundle loader、batch runner、selection helper、selection profile helper、named profile catalog の実装 anchor。
- `crates/mir-semantics/examples/current_l2_emit_detached_bundle.rs`
  - detached validation loop の bundle-first artifact を出す non-production emitter sketch。
- `crates/mir-semantics/examples/current_l2_emit_detached_aggregate.rs`
  - detached validation loop の aggregate summary artifact を出す non-production emitter sketch。
- `scripts/current_l2_diff_detached_artifacts.py`
  - detached artifact の payload core だけを比較する non-production helper。
- `scripts/current_l2_diff_detached_aggregates.py`
  - aggregate artifact の `summary_core` だけを比較する non-production helper。
- `scripts/current_l2_detached_loop.py`
  - bundle-first emitter、aggregate emitter、bundle diff helper、aggregate diff helper をつなぎ、artifact 保存と compare を最小で回す non-production wrapper。
- `scripts/current_l2_scaffold_fixture.py`
  - fixture authoring の boilerplate だけを `target/current-l2-fixture-scaffolds/` 下へ出す non-production scaffold helper。
- `scripts/current_l2_detached_loop.py`
  - detached validation loop の non-production wrapper。
  - `smoke-fixture` subcommand により、1 fixture を bundle export / optional reference compare / single-fixture aggregate smoke までまとめて回せる。
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
  - current L2 parser-free PoC の public behavior coverage を置く。

これらの code anchor の current status / call chain / docs/tests/code boundary は `plan/07-parser-free-poc-stack.md` と `plan/09-helper-stack-and-responsibility-map.md` に整理する。

## 用語と参照の編集方針

- 日本語正本として統一する語:
  - `safe evolution` は「安全な進化」と書く。
  - `shared space / shared state` は「共有空間 / 共有状態」と書く。
  - `control plane` は各文書の初出で「制御プレーン（control plane）」と併記してよい。
- formal token / 原語保持を優先する語:
  - `downstream addition`
    - 文脈により `downstream-only patching` や `leaf-style patching` といった局所表現を使う場合があるが、既定の進化方向を指す語幹として扱う。
  - `compatibility-preserving overlay`
    - 日本語 prose では「互換性を保つ overlay」と説明してよいが、formal token としてはこの形を優先する。
  - `wrap`
    - 日本語 prose では「包む」と書いてよいが、legacy integration や boundary operation を指すときは `wrap` を残してよい。
  - `fabric`
  - `kernel`
  - `runtime`
- 入口文書は、完全な文書マップと用語方針については `00-document-map.md` を、確定済み判断については `12-decision-register.md` を参照先として明示する。
