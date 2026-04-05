# plan/90 — source traceability

## 目的

この文書は、`plan/` 各ファイルの主根拠がどこにあるかを source file / report 単位で追跡できるようにする。
完全な line-by-line trace ではなく、主要 section ごとの traceability を与える。

## traceability table

| plan | 主な根拠 source |
|---|---|
| `plan/00-index.md` | `README.md`、`Documentation.md`、`specs/00-document-map.md`、`specs/11-roadmap-and-workstreams.md` |
| `plan/01-status-at-a-glance.md` | `Documentation.md`、`specs/10-open-questions.md`、`specs/11-roadmap-and-workstreams.md`、`specs/12-decision-register.md`、`docs/reports/0077`〜`0084`、`0155`、`0156`、`0157`、`0158`、`0159`、`0160`、`0161`、`0162` |
| `plan/02-system-overview-and-positioning.md` | `specs/02-system-overview.md`、`specs/03-layer-model.md`、`specs/05-mirrorea-fabric.md`、`specs/06-prismcascade-positioning.md`、`specs/07-typed-effects-wiring-platform.md`、`specs/08-cross-system-relations.md` |
| `plan/03-decision-strengths-and-boundaries.md` | `specs/01-charter-and-decision-levels.md`、`specs/12-decision-register.md`、`AGENTS.md` |
| `plan/04-core-semantics-current-l2.md` | `specs/04-mir-core.md`、`specs/09-invariants-and-constraints.md`、`specs/10-open-questions.md`、`docs/reports/0018`〜`0046` |
| `plan/05-fallback-lease-and-chain-semantics.md` | `docs/reports/0018`〜`0023`、`0037`、`0039`、`0043`、`0045`、`0121`、`specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md`、fixtures `e3/e6/e7/e8/e9` |
| `plan/06-surface-notation-status.md` | `specs/examples/01-current-l2-surface-syntax-candidates.md`、`specs/examples/29-current-l2-first-parser-subset-inventory.md`、`docs/reports/0025`、`0026`、`0028`、`0029`、`0030`、`0032`、`0034`、`0079`〜`0084`、`0132`、`0133` |
| `plan/07-parser-free-poc-stack.md` | `specs/examples/02`〜`13`、`specs/examples/16-current-l2-detached-trace-audit-artifact-schema.md`、`specs/examples/17-current-l2-detached-exporter-entry-comparison.md`、`specs/examples/18-current-l2-bundle-first-detached-payload-context-split.md`、`specs/examples/19-current-l2-host-plan-coverage-failure-placement.md`、`specs/examples/20-current-l2-host-plan-coverage-failure-bundle-failure-artifact-schema.md`、`specs/examples/21-current-l2-host-plan-coverage-failure-aggregate-connection.md`、`specs/examples/22-current-l2-host-plan-coverage-failure-aggregate-histogram-migration.md`、`specs/examples/23-current-l2-detached-export-loop-consolidation.md`、`specs/examples/24-current-l2-detached-export-storage-and-aggregate-api.md`、`specs/examples/25-current-l2-detached-aggregate-emitter-sketch.md`、`specs/examples/26-current-l2-detached-aggregate-compare-helper.md`、`specs/examples/27-current-l2-fixture-scaffold-helper.md`、`specs/examples/28-current-l2-detached-fixture-validation-loop-helper.md`、`specs/examples/31-current-l2-detached-aggregate-transform-helper.md`、`specs/examples/32-current-l2-static-gate-artifact-loop.md`、`specs/examples/33-current-l2-checked-static-reasons-carrier.md`、`specs/examples/34-current-l2-static-reason-code-entry-criteria.md`、`specs/examples/35-current-l2-detached-static-reason-code-mirror.md`、`specs/examples/36-current-l2-checked-reasons-authoring-assist.md`、`specs/examples/37-current-l2-detached-bundle-transform-helper.md`、`specs/examples/38-current-l2-static-reason-codes-authoring-assist.md`、`docs/reports/0047`〜`0077`、`0090`、`0092`、`0093`、`0094`、`0096`、`0098`、`0100`、`0103`、`0104`、`0106`、`0107`、`0108`、`0109`、`0118`、`0119`、`0122`、`0123`、`0124`、`0130`、`0131`、`0137`、`0138`、`0139`、`0140`、`0141`、`0142`、`0143`、`0144`、`0145`、`0146`、`0150`、`0151`、`0153`、`0154`、`0159`、`0160`、`0161`、`0162`、`crates/mir-semantics/src/lib.rs`、`crates/mir-semantics/src/harness.rs`、`crates/mir-semantics/examples/current_l2_emit_detached_bundle.rs`、`crates/mir-semantics/examples/current_l2_emit_detached_aggregate.rs`、`crates/mir-semantics/examples/current_l2_emit_static_gate.rs`、`crates/mir-semantics/examples/support/current_l2_detached_bundle_support.rs`、`crates/mir-semantics/examples/support/current_l2_detached_aggregate_support.rs`、`crates/mir-semantics/examples/support/current_l2_static_gate_support.rs`、`scripts/current_l2_checked_reasons_assist.py`、`scripts/current_l2_reason_codes_assist.py`、`scripts/current_l2_diff_detached_artifacts.py`、`scripts/current_l2_diff_detached_aggregates.py`、`scripts/current_l2_diff_static_gate_artifacts.py`、`scripts/current_l2_detached_loop.py`、`scripts/current_l2_scaffold_fixture.py` |
| `plan/08-representative-programs-and-fixtures.md` | `specs/examples/00`、`specs/examples/02`、`specs/examples/04`、`specs/examples/33-current-l2-checked-static-reasons-carrier.md`、fixtures `crates/mir-ast/tests/fixtures/current-l2/`、`docs/reports/0047`、`0049`、`0078`、`0121`、`0126`、`0127`、`0128`、`0129`、`0147`、`0148`、`0155`、`0156`、`0157`、`0158`、`0159`、`0160`、`0161`、`0162` |
| `plan/09-helper-stack-and-responsibility-map.md` | `specs/examples/09`〜`13`、`specs/examples/17-current-l2-detached-exporter-entry-comparison.md`、`specs/examples/18-current-l2-bundle-first-detached-payload-context-split.md`、`specs/examples/19-current-l2-host-plan-coverage-failure-placement.md`、`specs/examples/20-current-l2-host-plan-coverage-failure-bundle-failure-artifact-schema.md`、`specs/examples/21-current-l2-host-plan-coverage-failure-aggregate-connection.md`、`specs/examples/22-current-l2-host-plan-coverage-failure-aggregate-histogram-migration.md`、`specs/examples/23-current-l2-detached-export-loop-consolidation.md`、`specs/examples/24-current-l2-detached-export-storage-and-aggregate-api.md`、`specs/examples/25-current-l2-detached-aggregate-emitter-sketch.md`、`specs/examples/26-current-l2-detached-aggregate-compare-helper.md`、`specs/examples/27-current-l2-fixture-scaffold-helper.md`、`specs/examples/28-current-l2-detached-fixture-validation-loop-helper.md`、`specs/examples/31-current-l2-detached-aggregate-transform-helper.md`、`specs/examples/32-current-l2-static-gate-artifact-loop.md`、`specs/examples/35-current-l2-detached-static-reason-code-mirror.md`、`specs/examples/36-current-l2-checked-reasons-authoring-assist.md`、`specs/examples/37-current-l2-detached-bundle-transform-helper.md`、`specs/examples/38-current-l2-static-reason-codes-authoring-assist.md`、`docs/reports/0060`〜`0077`、`0094`、`0096`、`0098`、`0100`、`0103`、`0104`、`0106`、`0107`、`0108`、`0109`、`0118`、`0119`、`0122`、`0123`、`0124`、`0130`、`0131`、`0137`、`0138`、`0139`、`0140`、`0145`、`0146`、`0150`、`0153`、`0154`、`crates/mir-semantics/src/harness.rs`、`crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`、`crates/mir-semantics/examples/current_l2_emit_detached_bundle.rs`、`crates/mir-semantics/examples/current_l2_emit_detached_aggregate.rs`、`crates/mir-semantics/examples/current_l2_emit_static_gate.rs`、`crates/mir-semantics/examples/support/current_l2_detached_bundle_support.rs`、`crates/mir-semantics/examples/support/current_l2_detached_aggregate_support.rs`、`crates/mir-semantics/examples/support/current_l2_static_gate_support.rs`、`scripts/current_l2_checked_reasons_assist.py`、`scripts/current_l2_reason_codes_assist.py`、`scripts/current_l2_diff_detached_artifacts.py`、`scripts/current_l2_diff_detached_aggregates.py`、`scripts/current_l2_diff_static_gate_artifacts.py`、`scripts/current_l2_detached_loop.py`、`scripts/current_l2_scaffold_fixture.py` |
| `plan/10-roadmap-overall.md` | `specs/11-roadmap-and-workstreams.md`、`specs/10-open-questions.md`、`specs/12-decision-register.md`、`Documentation.md` |
| `plan/11-roadmap-near-term.md` | `specs/10-open-questions.md`、`specs/11-roadmap-and-workstreams.md`、`specs/examples/16-current-l2-detached-trace-audit-artifact-schema.md`、`specs/examples/17-current-l2-detached-exporter-entry-comparison.md`、`specs/examples/18-current-l2-bundle-first-detached-payload-context-split.md`、`specs/examples/19-current-l2-host-plan-coverage-failure-placement.md`、`specs/examples/20-current-l2-host-plan-coverage-failure-bundle-failure-artifact-schema.md`、`specs/examples/21-current-l2-host-plan-coverage-failure-aggregate-connection.md`、`specs/examples/22-current-l2-host-plan-coverage-failure-aggregate-histogram-migration.md`、`specs/examples/23-current-l2-detached-export-loop-consolidation.md`、`specs/examples/24-current-l2-detached-export-storage-and-aggregate-api.md`、`specs/examples/25-current-l2-detached-aggregate-emitter-sketch.md`、`specs/examples/26-current-l2-detached-aggregate-compare-helper.md`、`specs/examples/27-current-l2-fixture-scaffold-helper.md`、`specs/examples/28-current-l2-detached-fixture-validation-loop-helper.md`、`specs/examples/29-current-l2-first-parser-subset-inventory.md`、`specs/examples/30-current-l2-first-checker-cut-entry-criteria.md`、`specs/examples/31-current-l2-detached-aggregate-transform-helper.md`、`specs/examples/32-current-l2-static-gate-artifact-loop.md`、`specs/examples/33-current-l2-checked-static-reasons-carrier.md`、`specs/examples/34-current-l2-static-reason-code-entry-criteria.md`、`specs/examples/35-current-l2-detached-static-reason-code-mirror.md`、`specs/examples/36-current-l2-checked-reasons-authoring-assist.md`、`specs/examples/37-current-l2-detached-bundle-transform-helper.md`、`specs/examples/38-current-l2-static-reason-codes-authoring-assist.md`、`plan/15-current-l2-fixture-authoring-template.md`、`docs/reports/0054`、`0056`、`0059`、`0060`、`0062`、`0077`〜`0084`、`0089`、`0090`、`0092`、`0093`、`0094`、`0096`、`0098`、`0100`、`0103`、`0104`、`0106`、`0107`、`0108`、`0109`、`0118`、`0119`、`0122`、`0123`、`0124`、`0130`、`0131`、`0132`、`0133`、`0135`、`0136`、`0137`、`0138`、`0139`、`0140`、`0141`、`0142`、`0143`、`0144`、`0145`、`0146`、`0147`、`0148`、`0149`、`0150`、`0153`、`0154`、`0155`、`0156`、`0157`、`0158`、`0159`、`0160`、`0161`、`0162` |
| `plan/12-open-problems-and-risks.md` | `specs/04-mir-core.md`、`specs/05-mirrorea-fabric.md`、`specs/10-open-questions.md`、`specs/11-roadmap-and-workstreams.md`、`specs/12-decision-register.md`、`specs/examples/16-current-l2-detached-trace-audit-artifact-schema.md`、`specs/examples/17-current-l2-detached-exporter-entry-comparison.md`、`specs/examples/18-current-l2-bundle-first-detached-payload-context-split.md`、`specs/examples/19-current-l2-host-plan-coverage-failure-placement.md`、`specs/examples/20-current-l2-host-plan-coverage-failure-bundle-failure-artifact-schema.md`、`specs/examples/21-current-l2-host-plan-coverage-failure-aggregate-connection.md`、`specs/examples/22-current-l2-host-plan-coverage-failure-aggregate-histogram-migration.md`、`specs/examples/23-current-l2-detached-export-loop-consolidation.md`、`specs/examples/24-current-l2-detached-export-storage-and-aggregate-api.md`、`specs/examples/25-current-l2-detached-aggregate-emitter-sketch.md`、`specs/examples/26-current-l2-detached-aggregate-compare-helper.md`、`specs/examples/27-current-l2-fixture-scaffold-helper.md`、`specs/examples/28-current-l2-detached-fixture-validation-loop-helper.md`、`specs/examples/29-current-l2-first-parser-subset-inventory.md`、`specs/examples/30-current-l2-first-checker-cut-entry-criteria.md`、`specs/examples/31-current-l2-detached-aggregate-transform-helper.md`、`specs/examples/32-current-l2-static-gate-artifact-loop.md`、`specs/examples/33-current-l2-checked-static-reasons-carrier.md`、`specs/examples/34-current-l2-static-reason-code-entry-criteria.md`、`specs/examples/35-current-l2-detached-static-reason-code-mirror.md`、`specs/examples/37-current-l2-detached-bundle-transform-helper.md`、`plan/15-current-l2-fixture-authoring-template.md`、`docs/reports/0067`、`0079`〜`0084`、`0089`、`0090`、`0092`、`0093`、`0094`、`0096`、`0098`、`0100`、`0103`、`0104`、`0106`、`0107`、`0108`、`0109`、`0118`、`0119`、`0122`、`0123`、`0124`、`0130`、`0131`、`0132`、`0133`、`0135`、`0136`、`0137`、`0138`、`0139`、`0140`、`0141`、`0142`、`0143`、`0144`、`0145`、`0146`、`0149`、`0153`、`0154` |
| `plan/13-heavy-future-workstreams.md` | `specs/11-roadmap-and-workstreams.md`、`specs/10-open-questions.md`、`specs/07-typed-effects-wiring-platform.md`、`specs/08-cross-system-relations.md`、`specs/examples/29-current-l2-first-parser-subset-inventory.md`、`specs/examples/30-current-l2-first-checker-cut-entry-criteria.md`、`docs/reports/0132`、`0133`、`0135`、`0136` |
| `plan/14-glossary-and-boundary-rules.md` | `docs/reports/0017-terminology-audit-and-cross-reference-alignment.md`、`specs/00-document-map.md`、`specs/04-mir-core.md`、`specs/examples/09`〜`13` |
| `plan/15-current-l2-fixture-authoring-template.md` | `specs/examples/02-current-l2-ast-fixture-schema.md`、`specs/examples/07-current-l2-host-stub-harness.md`、`specs/examples/08-current-l2-host-plan-schema.md`、`specs/examples/16-current-l2-detached-trace-audit-artifact-schema.md`、`specs/examples/17-current-l2-detached-exporter-entry-comparison.md`、`specs/examples/18-current-l2-bundle-first-detached-payload-context-split.md`、`specs/examples/23-current-l2-detached-export-loop-consolidation.md`、`specs/examples/24-current-l2-detached-export-storage-and-aggregate-api.md`、`specs/examples/25-current-l2-detached-aggregate-emitter-sketch.md`、`specs/examples/26-current-l2-detached-aggregate-compare-helper.md`、`specs/examples/27-current-l2-fixture-scaffold-helper.md`、`specs/examples/28-current-l2-detached-fixture-validation-loop-helper.md`、`specs/examples/32-current-l2-static-gate-artifact-loop.md`、`specs/examples/33-current-l2-checked-static-reasons-carrier.md`、`specs/examples/35-current-l2-detached-static-reason-code-mirror.md`、`specs/examples/36-current-l2-checked-reasons-authoring-assist.md`、`specs/examples/38-current-l2-static-reason-codes-authoring-assist.md`、`plan/08-representative-programs-and-fixtures.md`、`docs/reports/0106`、`docs/reports/0107`、`docs/reports/0118`、`docs/reports/0119`、`docs/reports/0122`、`docs/reports/0123`、`docs/reports/0124`、`docs/reports/0130`、`docs/reports/0131`、`docs/reports/0139`、`docs/reports/0140`、`docs/reports/0141`、`docs/reports/0142`、`docs/reports/0143`、`docs/reports/0145`、`docs/reports/0146`、`docs/reports/0147`、`docs/reports/0148`、`docs/reports/0149`、`docs/reports/0150`、`docs/reports/0151`、`docs/reports/0155`、`0156`、`0157`、`0158`、`0159`、`0160`、`0161`、`0162`、`crates/mir-ast/tests/fixtures/current-l2/`、`crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`、`scripts/current_l2_checked_reasons_assist.py`、`scripts/current_l2_reason_codes_assist.py`、`scripts/current_l2_detached_loop.py`、`scripts/current_l2_diff_detached_aggregates.py`、`scripts/current_l2_diff_static_gate_artifacts.py`、`scripts/current_l2_scaffold_fixture.py` |
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
- `0139`
- `0140`
- `0141`
- `0142`
- `0143`
- `0144`
- `0145`
- `0146`
- `0149`

### 2026-04-05 readiness scan addendum

- `plan/07-parser-free-poc-stack.md`、`plan/09-helper-stack-and-responsibility-map.md`、`plan/11-roadmap-near-term.md`、`plan/15-current-l2-fixture-authoring-template.md` の今回更新分は、追加で次を主根拠にする。
  - `specs/examples/39-current-l2-static-reason-code-readiness-scan.md`
  - `docs/reports/0166-static-reason-code-readiness-scan.md`
  - `docs/reports/0167-review-static-reason-code-readiness-scan.md`
  - `scripts/current_l2_reason_code_readiness.py`
  - `scripts/current_l2_detached_loop.py`
  - `scripts/tests/test_current_l2_reason_code_readiness.py`
  - current fixture corpus `crates/mir-ast/tests/fixtures/current-l2/`

### 2026-04-05 first typed static reason family addendum

- `plan/11-roadmap-near-term.md`、`plan/12-open-problems-and-risks.md`、`progress.md` の今回更新分は、追加で次を主根拠にする。
  - `specs/examples/40-current-l2-first-typed-static-reason-family-selection.md`
  - `specs/examples/41-current-l2-first-typed-static-reason-family-carrier-cut.md`
  - `specs/examples/42-current-l2-second-typed-static-reason-family-actualization.md`
  - `specs/examples/43-current-l2-complete-stable-static-reason-tranche.md`
  - `docs/reports/0168-first-typed-static-reason-family-selection.md`
  - `docs/reports/0169-review-first-typed-static-reason-family-selection.md`
  - `crates/mir-semantics/src/lib.rs`
  - `crates/mir-semantics/src/harness.rs`
  - `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
  - `crates/mir-semantics/tests/current_l2_static_gate_support.rs`
  - `scripts/current_l2_reason_codes_assist.py`
  - `scripts/current_l2_reason_code_readiness.py`
  - `scripts/tests/test_current_l2_reason_codes_assist.py`
  - `scripts/tests/test_current_l2_reason_code_readiness.py`
  - current fixture corpus `crates/mir-ast/tests/fixtures/current-l2/e4-malformed-lineage.json`
  - current fixture corpus `crates/mir-ast/tests/fixtures/current-l2/e5-underdeclared-lineage.json`
  - current fixture corpus `crates/mir-ast/tests/fixtures/current-l2/e12-underdeclared-target-missing.json`
  - current fixture corpus `crates/mir-ast/tests/fixtures/current-l2/e13-malformed-capability-strengthening.json`
  - current fixture corpus `crates/mir-ast/tests/fixtures/current-l2/e16-malformed-missing-chain-head-option.json`
  - current fixture corpus `crates/mir-ast/tests/fixtures/current-l2/e17-malformed-missing-predecessor-option.json`
  - current fixture corpus `crates/mir-ast/tests/fixtures/current-l2/e18-malformed-missing-successor-option.json`
  - current fixture corpus `crates/mir-ast/tests/fixtures/current-l2/e19-malformed-target-mismatch.json`

### 2026-04-05 checked reasons coexistence addendum

- `plan/07-parser-free-poc-stack.md`、`plan/09-helper-stack-and-responsibility-map.md`、`plan/11-roadmap-near-term.md`、`plan/12-open-problems-and-risks.md`、`plan/15-current-l2-fixture-authoring-template.md`、`progress.md` の今回更新分は、追加で次を主根拠にする。
  - `specs/examples/44-current-l2-checked-reasons-coexistence-and-shrink-policy.md`
  - `docs/reports/0170-checked-reasons-coexistence-and-shrink-policy.md`
  - `docs/reports/0171-review-checked-reasons-coexistence-and-shrink-policy.md`
  - `scripts/current_l2_reason_code_readiness.py`
  - `scripts/tests/test_current_l2_reason_code_readiness.py`
  - `scripts/current_l2_detached_loop.py`
  - current fixture corpus `crates/mir-ast/tests/fixtures/current-l2/`

### 2026-04-05 first checker cut baseline addendum

- `plan/07-parser-free-poc-stack.md`、`plan/09-helper-stack-and-responsibility-map.md`、`plan/11-roadmap-near-term.md`、`plan/12-open-problems-and-risks.md`、`plan/13-heavy-future-workstreams.md`、`progress.md` の今回更新分は、追加で次を主根拠にする。
  - `specs/examples/45-current-l2-first-checker-cut-regression-baseline.md`
  - `docs/reports/0172-first-checker-cut-regression-baseline.md`
  - `docs/reports/0173-review-first-checker-cut-regression-baseline.md`
  - `scripts/current_l2_reason_code_readiness.py`
  - `scripts/tests/test_current_l2_reason_code_readiness.py`
  - `scripts/current_l2_detached_loop.py`
  - current fixture corpus `crates/mir-ast/tests/fixtures/current-l2/`

### 2026-04-05 same-lineage first checker spike addendum

- `plan/07-parser-free-poc-stack.md`、`plan/09-helper-stack-and-responsibility-map.md`、`plan/11-roadmap-near-term.md`、`plan/12-open-problems-and-risks.md`、`plan/13-heavy-future-workstreams.md`、`plan/15-current-l2-fixture-authoring-template.md`、`progress.md` の今回更新分は、追加で次を主根拠にする。
  - `specs/examples/46-current-l2-same-lineage-first-checker-spike.md`
  - `docs/reports/0174-same-lineage-first-checker-spike.md`
  - `docs/reports/0175-review-same-lineage-first-checker-spike.md`
  - `scripts/current_l2_same_lineage_checker.py`
  - `scripts/tests/test_current_l2_same_lineage_checker.py`
  - `scripts/current_l2_detached_loop.py`
  - `scripts/tests/test_current_l2_static_gate_loop.py`
  - current fixture corpus `crates/mir-ast/tests/fixtures/current-l2/`

### 2026-04-05 missing-option second checker spike addendum

- `plan/07-parser-free-poc-stack.md`、`plan/09-helper-stack-and-responsibility-map.md`、`plan/11-roadmap-near-term.md`、`plan/12-open-problems-and-risks.md`、`plan/13-heavy-future-workstreams.md`、`plan/15-current-l2-fixture-authoring-template.md`、`progress.md` の今回更新分は、追加で次を主根拠にする。
  - `specs/examples/47-current-l2-missing-option-second-checker-spike.md`
  - `docs/reports/0176-missing-option-second-checker-spike.md`
  - `docs/reports/0177-review-missing-option-second-checker-spike.md`
  - `scripts/current_l2_missing_option_checker.py`
  - `scripts/tests/test_current_l2_missing_option_checker.py`
  - `scripts/current_l2_detached_loop.py`
  - `scripts/tests/test_current_l2_static_gate_loop.py`
  - current fixture corpus `crates/mir-ast/tests/fixtures/current-l2/`

### 2026-04-06 capability third checker spike addendum

- `plan/07-parser-free-poc-stack.md`、`plan/08-representative-programs-and-fixtures.md`、`plan/09-helper-stack-and-responsibility-map.md`、`plan/11-roadmap-near-term.md`、`plan/12-open-problems-and-risks.md`、`plan/13-heavy-future-workstreams.md`、`plan/15-current-l2-fixture-authoring-template.md`、`progress.md` の今回更新分は、追加で次を主根拠にする。
  - `specs/examples/45-current-l2-first-checker-cut-regression-baseline.md`
  - `specs/examples/48-current-l2-capability-third-checker-spike.md`
  - `docs/reports/0178-capability-floor-third-checker-spike.md`
  - `docs/reports/0179-review-capability-floor-third-checker-spike.md`
  - `scripts/current_l2_capability_checker.py`
  - `scripts/tests/test_current_l2_capability_checker.py`
  - `scripts/current_l2_detached_loop.py`
  - `scripts/tests/test_current_l2_static_gate_loop.py`
  - `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
  - `crates/mir-semantics/tests/current_l2_static_gate_support.rs`
  - current fixture corpus `crates/mir-ast/tests/fixtures/current-l2/e13-malformed-capability-strengthening.json`
  - current fixture corpus `crates/mir-ast/tests/fixtures/current-l2/e20-malformed-late-capability-strengthening.json`

### 2026-04-06 shared family checker support helper addendum

- specs:
  - `specs/examples/49-current-l2-shared-family-checker-support-helper.md`
- reports:
  - `docs/reports/0180-shared-family-checker-support-helper.md`
  - `docs/reports/0181-review-shared-family-checker-support-helper.md`
- scripts / tests:
  - `scripts/current_l2_family_checker_support.py`
  - `scripts/current_l2_same_lineage_checker.py`
  - `scripts/current_l2_missing_option_checker.py`
  - `scripts/current_l2_capability_checker.py`
  - `scripts/tests/test_current_l2_family_checker_support.py`
  - `scripts/tests/test_current_l2_same_lineage_checker.py`
  - `scripts/tests/test_current_l2_missing_option_checker.py`
  - `scripts/tests/test_current_l2_capability_checker.py`

### 2026-04-06 generic family checker entry comparison addendum

- specs:
  - `specs/examples/50-current-l2-generic-family-checker-entry-comparison.md`
- reports:
  - `docs/reports/0182-generic-family-checker-entry-comparison.md`
  - `docs/reports/0183-review-generic-family-checker-entry-comparison.md`

### 2026-04-06 try-body atomic-cut frontier runtime fixture addendum

- `specs/examples/00-representative-mir-programs.md`、`specs/examples/02-current-l2-ast-fixture-schema.md`、`specs/examples/04-current-l2-step-semantics.md`、`specs/examples/09-current-l2-bundle-loader.md`、`specs/examples/10-current-l2-batch-runner.md`、`specs/examples/11-current-l2-selection-helper.md`、`plan/08-representative-programs-and-fixtures.md`、`plan/11-roadmap-near-term.md`、`progress.md` の今回更新分は、追加で次を主根拠にする。
  - `crates/mir-ast/tests/fixtures/current-l2/e21-try-atomic-cut-frontier.json`
  - `crates/mir-ast/tests/fixtures/current-l2/e21-try-atomic-cut-frontier.host-plan.json`
  - `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
  - `docs/reports/0184-try-atomic-cut-frontier-runtime-fixture.md`
  - `docs/reports/0185-review-try-atomic-cut-frontier-runtime-fixture.md`

### 2026-04-06 nested-place atomic-cut mismatch and checker-boundary addendum

- `specs/examples/00-representative-mir-programs.md`、`specs/examples/02-current-l2-ast-fixture-schema.md`、`specs/examples/04-current-l2-step-semantics.md`、`specs/examples/09-current-l2-bundle-loader.md`、`specs/examples/10-current-l2-batch-runner.md`、`specs/examples/11-current-l2-selection-helper.md`、`specs/examples/30-current-l2-first-checker-cut-entry-criteria.md`、`specs/examples/51-current-l2-try-rollback-structural-floor-and-restore-scope.md`、`plan/01-status-at-a-glance.md`、`plan/04-core-semantics-current-l2.md`、`plan/08-representative-programs-and-fixtures.md`、`plan/11-roadmap-near-term.md`、`plan/12-open-problems-and-risks.md`、`progress.md` の今回更新分は、追加で次を主根拠にする。
  - `crates/mir-ast/tests/fixtures/current-l2/e22-try-atomic-cut-place-mismatch.json`
  - `crates/mir-ast/tests/fixtures/current-l2/e22-try-atomic-cut-place-mismatch.host-plan.json`
  - `crates/mir-semantics/src/lib.rs`
  - `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
  - `docs/reports/0186-try-rollback-structural-floor-and-restore-scope.md`
  - `docs/reports/0187-review-try-rollback-structural-floor-and-restore-scope.md`

### 2026-04-06 try-rollback locality smoke convenience addendum

- `plan/07-parser-free-poc-stack.md`、`plan/09-helper-stack-and-responsibility-map.md`、`progress.md` の今回更新分は、追加で次を主根拠にする。
  - `scripts/current_l2_detached_loop.py`
  - `scripts/tests/test_current_l2_detached_loop.py`
  - `docs/reports/0188-try-rollback-locality-smoke-convenience.md`

### 2026-04-06 try-rollback fourth checker spike comparison addendum

- `Documentation.md`、`specs/00-document-map.md`、`plan/11-roadmap-near-term.md`、`plan/12-open-problems-and-risks.md`、`progress.md` の今回更新分は、追加で次を主根拠にする。
  - `specs/examples/52-current-l2-try-rollback-fourth-checker-spike-comparison.md`
  - `specs/examples/51-current-l2-try-rollback-structural-floor-and-restore-scope.md`

### 2026-04-06 try-rollback AST structural helper entry-criteria addendum

- `Documentation.md`、`specs/00-document-map.md`、`plan/11-roadmap-near-term.md`、`plan/12-open-problems-and-risks.md`、`progress.md` の今回更新分は、追加で次を主根拠にする。
  - `specs/examples/29-current-l2-first-parser-subset-inventory.md`
  - `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md`
  - `specs/examples/51-current-l2-try-rollback-structural-floor-and-restore-scope.md`
  - `specs/examples/52-current-l2-try-rollback-fourth-checker-spike-comparison.md`
  - `specs/examples/53-current-l2-try-rollback-ast-structural-helper-entry-criteria.md`
  - `docs/reports/0192-try-rollback-ast-structural-helper-entry-criteria.md`
  - `docs/reports/0193-review-try-rollback-ast-structural-helper-entry-criteria.md`

### 2026-04-06 try-rollback structural malformed source placement addendum

- `Documentation.md`、`specs/00-document-map.md`、`plan/11-roadmap-near-term.md`、`plan/12-open-problems-and-risks.md`、`progress.md` の今回更新分は、追加で次を主根拠にする。
  - `specs/examples/29-current-l2-first-parser-subset-inventory.md`
  - `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md`
  - `specs/examples/53-current-l2-try-rollback-ast-structural-helper-entry-criteria.md`
  - `specs/examples/54-current-l2-try-rollback-structural-malformed-source-placement.md`
  - `docs/reports/0194-try-rollback-structural-malformed-source-placement.md`
  - `docs/reports/0195-review-try-rollback-structural-malformed-source-placement.md`

### 2026-04-06 try-rollback malformed static family actualization addendum

- `Documentation.md`、`specs/00-document-map.md`、`plan/11-roadmap-near-term.md`、`plan/12-open-problems-and-risks.md`、`progress.md` の今回更新分は、追加で次を主根拠にする。
  - `specs/examples/51-current-l2-try-rollback-structural-floor-and-restore-scope.md`
  - `specs/examples/52-current-l2-try-rollback-fourth-checker-spike-comparison.md`
  - `specs/examples/53-current-l2-try-rollback-ast-structural-helper-entry-criteria.md`
  - `specs/examples/54-current-l2-try-rollback-structural-malformed-source-placement.md`
  - `specs/examples/55-current-l2-try-rollback-malformed-static-family-actualization.md`
  - `docs/reports/0196-try-rollback-malformed-static-family-actualization.md`
  - `docs/reports/0197-review-try-rollback-malformed-static-family-actualization.md`

### 2026-04-06 try-rollback AST helper compare contract addendum

- `Documentation.md`、`specs/00-document-map.md`、`plan/11-roadmap-near-term.md`、`plan/12-open-problems-and-risks.md`、`progress.md` の今回更新分は、追加で次を主根拠にする。
  - `specs/examples/52-current-l2-try-rollback-fourth-checker-spike-comparison.md`
  - `specs/examples/53-current-l2-try-rollback-ast-structural-helper-entry-criteria.md`
  - `specs/examples/54-current-l2-try-rollback-structural-malformed-source-placement.md`
  - `specs/examples/55-current-l2-try-rollback-malformed-static-family-actualization.md`
  - `specs/examples/56-current-l2-try-rollback-ast-helper-compare-contract.md`
  - `docs/reports/0198-try-rollback-ast-helper-compare-contract.md`
  - `docs/reports/0199-review-try-rollback-ast-helper-compare-contract.md`
  - `scripts/current_l2_family_checker_support.py`
  - `scripts/current_l2_same_lineage_checker.py`
  - `scripts/current_l2_missing_option_checker.py`
  - `scripts/current_l2_capability_checker.py`
  - `docs/reports/0190-try-rollback-fourth-checker-spike-comparison.md`
  - `docs/reports/0191-review-try-rollback-fourth-checker-spike-comparison.md`

### 2026-04-06 try-rollback AST helper expected-field-name addendum

- `Documentation.md`、`specs/00-document-map.md`、`plan/11-roadmap-near-term.md`、`plan/12-open-problems-and-risks.md`、`plan/15-current-l2-fixture-authoring-template.md`、`progress.md` の今回更新分は、追加で次を主根拠にする。
  - `specs/examples/56-current-l2-try-rollback-ast-helper-compare-contract.md`
  - `specs/examples/57-current-l2-try-rollback-ast-helper-expected-field-name.md`
  - `docs/reports/0200-try-rollback-ast-helper-expected-field-name.md`
  - `docs/reports/0201-review-try-rollback-ast-helper-expected-field-name.md`

### 2026-04-06 try-rollback AST helper detached-loop insertion addendum

- `Documentation.md`、`specs/00-document-map.md`、`plan/07-parser-free-poc-stack.md`、`plan/09-helper-stack-and-responsibility-map.md`、`plan/11-roadmap-near-term.md`、`plan/12-open-problems-and-risks.md`、`plan/15-current-l2-fixture-authoring-template.md`、`progress.md` の今回更新分は、追加で次を主根拠にする。
  - `specs/examples/28-current-l2-detached-fixture-validation-loop-helper.md`
  - `specs/examples/49-current-l2-shared-family-checker-support-helper.md`
  - `specs/examples/50-current-l2-generic-family-checker-entry-comparison.md`
  - `specs/examples/56-current-l2-try-rollback-ast-helper-compare-contract.md`
  - `specs/examples/57-current-l2-try-rollback-ast-helper-expected-field-name.md`
  - `specs/examples/58-current-l2-try-rollback-ast-helper-detached-loop-insertion.md`
  - `docs/reports/0202-try-rollback-ast-helper-detached-loop-insertion.md`
  - `docs/reports/0203-review-try-rollback-ast-helper-detached-loop-insertion.md`
  - `scripts/current_l2_detached_loop.py`
  - `crates/mir-semantics/examples/current_l2_emit_static_gate.rs`
  - `crates/mir-semantics/examples/support/current_l2_static_gate_support.rs`

### 2026-04-06 try-rollback AST helper structural-verdict carrier addendum

- `Documentation.md`、`specs/00-document-map.md`、`plan/11-roadmap-near-term.md`、`plan/12-open-problems-and-risks.md`、`plan/15-current-l2-fixture-authoring-template.md`、`progress.md` の今回更新分は、追加で次を主根拠にする。
  - `specs/examples/56-current-l2-try-rollback-ast-helper-compare-contract.md`
  - `specs/examples/57-current-l2-try-rollback-ast-helper-expected-field-name.md`
  - `specs/examples/58-current-l2-try-rollback-ast-helper-detached-loop-insertion.md`
  - `specs/examples/59-current-l2-try-rollback-ast-helper-structural-verdict-carrier.md`
  - `docs/reports/0204-try-rollback-ast-helper-structural-verdict-carrier.md`
  - `docs/reports/0205-review-try-rollback-ast-helper-structural-verdict-carrier.md`

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
