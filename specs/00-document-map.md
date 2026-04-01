# 00 — 文書マップ

この文書は、主要な関心事がそれぞれどこに書かれているかを読者または agent に示す。

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
  - ここにあるコード片は parser-ready な最終 syntax を固定するものではなく、規範文書の current reading を具体例として読むための companion として扱う。

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
