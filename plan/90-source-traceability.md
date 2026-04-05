# plan/90 — source traceability

## 目的

この文書は、`plan/` 各ファイルの主根拠がどこにあるかを source file / report 単位で追跡できるようにする。
完全な line-by-line trace ではなく、主要 section ごとの traceability を与える。

## traceability table

| plan | 主な根拠 source |
|---|---|
| `plan/00-index.md` | `README.md`、`Documentation.md`、`specs/00-document-map.md`、`specs/11-roadmap-and-workstreams.md` |
| `plan/01-status-at-a-glance.md` | `Documentation.md`、`specs/10-open-questions.md`、`specs/11-roadmap-and-workstreams.md`、`specs/12-decision-register.md`、`docs/reports/0077`〜`0084` |
| `plan/02-system-overview-and-positioning.md` | `specs/02-system-overview.md`、`specs/03-layer-model.md`、`specs/05-mirrorea-fabric.md`、`specs/06-prismcascade-positioning.md`、`specs/07-typed-effects-wiring-platform.md`、`specs/08-cross-system-relations.md` |
| `plan/03-decision-strengths-and-boundaries.md` | `specs/01-charter-and-decision-levels.md`、`specs/12-decision-register.md`、`AGENTS.md` |
| `plan/04-core-semantics-current-l2.md` | `specs/04-mir-core.md`、`specs/09-invariants-and-constraints.md`、`specs/10-open-questions.md`、`docs/reports/0018`〜`0046` |
| `plan/05-fallback-lease-and-chain-semantics.md` | `docs/reports/0018`〜`0023`、`0037`、`0039`、`0043`、`0045`、`specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md`、fixtures `e3/e6/e7/e8` |
| `plan/06-surface-notation-status.md` | `specs/examples/01-current-l2-surface-syntax-candidates.md`、`docs/reports/0025`、`0026`、`0028`、`0029`、`0030`、`0032`、`0034`、`0079`〜`0084` |
| `plan/07-parser-free-poc-stack.md` | `specs/examples/02`〜`13`、`specs/examples/16-current-l2-detached-trace-audit-artifact-schema.md`、`specs/examples/17-current-l2-detached-exporter-entry-comparison.md`、`specs/examples/18-current-l2-bundle-first-detached-payload-context-split.md`、`specs/examples/19-current-l2-host-plan-coverage-failure-placement.md`、`specs/examples/20-current-l2-host-plan-coverage-failure-bundle-failure-artifact-schema.md`、`specs/examples/21-current-l2-host-plan-coverage-failure-aggregate-connection.md`、`specs/examples/22-current-l2-host-plan-coverage-failure-aggregate-histogram-migration.md`、`specs/examples/23-current-l2-detached-export-loop-consolidation.md`、`specs/examples/24-current-l2-detached-export-storage-and-aggregate-api.md`、`docs/reports/0047`〜`0077`、`0090`、`0092`、`0093`、`0094`、`0096`、`0098`、`0100`、`0103`、`0104`、`0106`、`0107`、`0108`、`0109`、`crates/mir-semantics/src/lib.rs`、`crates/mir-semantics/src/harness.rs`、`crates/mir-semantics/examples/current_l2_emit_detached_bundle.rs`、`scripts/current_l2_diff_detached_artifacts.py`、`scripts/current_l2_detached_loop.py` |
| `plan/08-representative-programs-and-fixtures.md` | `specs/examples/00`、`specs/examples/02`、`specs/examples/04`、fixtures `crates/mir-ast/tests/fixtures/current-l2/`、`docs/reports/0047`、`0049`、`0078` |
| `plan/09-helper-stack-and-responsibility-map.md` | `specs/examples/09`〜`13`、`specs/examples/17-current-l2-detached-exporter-entry-comparison.md`、`specs/examples/18-current-l2-bundle-first-detached-payload-context-split.md`、`specs/examples/19-current-l2-host-plan-coverage-failure-placement.md`、`specs/examples/20-current-l2-host-plan-coverage-failure-bundle-failure-artifact-schema.md`、`specs/examples/21-current-l2-host-plan-coverage-failure-aggregate-connection.md`、`specs/examples/22-current-l2-host-plan-coverage-failure-aggregate-histogram-migration.md`、`specs/examples/23-current-l2-detached-export-loop-consolidation.md`、`specs/examples/24-current-l2-detached-export-storage-and-aggregate-api.md`、`docs/reports/0060`〜`0077`、`0094`、`0096`、`0098`、`0100`、`0103`、`0104`、`0106`、`0107`、`0108`、`0109`、`crates/mir-semantics/src/harness.rs`、`crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`、`crates/mir-semantics/examples/current_l2_emit_detached_bundle.rs`、`scripts/current_l2_diff_detached_artifacts.py`、`scripts/current_l2_detached_loop.py` |
| `plan/10-roadmap-overall.md` | `specs/11-roadmap-and-workstreams.md`、`specs/10-open-questions.md`、`specs/12-decision-register.md`、`Documentation.md` |
| `plan/11-roadmap-near-term.md` | `specs/10-open-questions.md`、`specs/11-roadmap-and-workstreams.md`、`specs/examples/16-current-l2-detached-trace-audit-artifact-schema.md`、`specs/examples/17-current-l2-detached-exporter-entry-comparison.md`、`specs/examples/18-current-l2-bundle-first-detached-payload-context-split.md`、`specs/examples/19-current-l2-host-plan-coverage-failure-placement.md`、`specs/examples/20-current-l2-host-plan-coverage-failure-bundle-failure-artifact-schema.md`、`specs/examples/21-current-l2-host-plan-coverage-failure-aggregate-connection.md`、`specs/examples/22-current-l2-host-plan-coverage-failure-aggregate-histogram-migration.md`、`specs/examples/23-current-l2-detached-export-loop-consolidation.md`、`specs/examples/24-current-l2-detached-export-storage-and-aggregate-api.md`、`plan/15-current-l2-fixture-authoring-template.md`、`docs/reports/0054`、`0056`、`0059`、`0060`、`0062`、`0077`〜`0084`、`0089`、`0090`、`0092`、`0093`、`0094`、`0096`、`0098`、`0100`、`0103`、`0104`、`0106`、`0107`、`0108`、`0109` |
| `plan/12-open-problems-and-risks.md` | `specs/04-mir-core.md`、`specs/05-mirrorea-fabric.md`、`specs/10-open-questions.md`、`specs/11-roadmap-and-workstreams.md`、`specs/12-decision-register.md`、`specs/examples/16-current-l2-detached-trace-audit-artifact-schema.md`、`specs/examples/17-current-l2-detached-exporter-entry-comparison.md`、`specs/examples/18-current-l2-bundle-first-detached-payload-context-split.md`、`specs/examples/19-current-l2-host-plan-coverage-failure-placement.md`、`specs/examples/20-current-l2-host-plan-coverage-failure-bundle-failure-artifact-schema.md`、`specs/examples/21-current-l2-host-plan-coverage-failure-aggregate-connection.md`、`specs/examples/22-current-l2-host-plan-coverage-failure-aggregate-histogram-migration.md`、`specs/examples/23-current-l2-detached-export-loop-consolidation.md`、`specs/examples/24-current-l2-detached-export-storage-and-aggregate-api.md`、`plan/15-current-l2-fixture-authoring-template.md`、`docs/reports/0067`、`0079`〜`0084`、`0089`、`0090`、`0092`、`0093`、`0094`、`0096`、`0098`、`0100`、`0103`、`0104`、`0106`、`0107`、`0108`、`0109` |
| `plan/13-heavy-future-workstreams.md` | `specs/11-roadmap-and-workstreams.md`、`specs/10-open-questions.md`、`specs/07-typed-effects-wiring-platform.md`、`specs/08-cross-system-relations.md` |
| `plan/14-glossary-and-boundary-rules.md` | `docs/reports/0017-terminology-audit-and-cross-reference-alignment.md`、`specs/00-document-map.md`、`specs/04-mir-core.md`、`specs/examples/09`〜`13` |
| `plan/15-current-l2-fixture-authoring-template.md` | `specs/examples/02-current-l2-ast-fixture-schema.md`、`specs/examples/07-current-l2-host-stub-harness.md`、`specs/examples/08-current-l2-host-plan-schema.md`、`specs/examples/16-current-l2-detached-trace-audit-artifact-schema.md`、`specs/examples/17-current-l2-detached-exporter-entry-comparison.md`、`specs/examples/18-current-l2-bundle-first-detached-payload-context-split.md`、`specs/examples/23-current-l2-detached-export-loop-consolidation.md`、`specs/examples/24-current-l2-detached-export-storage-and-aggregate-api.md`、`plan/08-representative-programs-and-fixtures.md`、`docs/reports/0106`、`docs/reports/0107`、`crates/mir-ast/tests/fixtures/current-l2/`、`crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`、`scripts/current_l2_detached_loop.py` |
| `plan/91-maintenance-rules.md` | `AGENTS.md`、`Documentation.md`、`specs/00-document-map.md`、report policy、helper boundary reports `0071`〜`0077` |

## 実装 anchor

`plan/` は docs だけでなく code anchor にも依拠する。
特に current L2 parser-free PoC stack では次が重要である。

- `crates/mir-semantics/src/lib.rs`
  - interpreter skeleton と evaluation entry point
- `crates/mir-semantics/src/harness.rs`
  - host harness、bundle / batch / selection / profile / named profile helper stack
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
  - public behavior coverage
- `crates/mir-ast/tests/fixtures/current-l2/`
  - machine-check 用 representative fixture と `.host-plan.json` sidecar

## report chain の読み方

大きな論点ごとに、次の report chain が主根拠になる。

### fallback / `lease`

- `0018`〜`0023`
- `0037`
- `0039`
- `0043`
- `0045`
- `0078`
- `0079`
- `0080`
- `0081`
- `0082`
- `0083`
- `0084`

### parser-free PoC stack

- `0047`
- `0048`
- `0049`
- `0051`
- `0054`
- `0056`
- `0059`
- `0060`
- `0062`
- `0063`
- `0064`
- `0066`
- `0069`
- `0077`
- `0089`
- `0090`
- `0092`
- `0094`
- `0093`
- `0096`
- `0098`
- `0100`
- `0101`
- `0102`
- `0103`
- `0104`
- `0105`
- `0106`
- `0107`
- `0108`
- `0109`

### named profile catalog と mirror boundary

- `0066`
- `0067`
- `0068`
- `0069`
- `0070`
- `0071`
- `0072`
- `0073`
- `0074`
- `0075`
- `0076`

## update rule

`plan/` の section を更新したら、この文書の対応する trace も必要に応じて更新する。
特に新しい anchor 文書や report chain が増えた場合は、ここへ追記する。
