# Report 0106 — detached exporter consolidation sprint

- Date: 2026-04-03
- Author / agent: Codex
- Scope: current L2 parser-free PoC の detached export loop を docs-only judgment から non-production の最小 loop attachment まで接続する
- Decision levels touched: L2

## 1. Objective

current L2 parser-free PoC 基盤を前提に、detached exporter chain の docs-only judgment を 1 箇所へ統合しつつ、

- bundle-first の tiny exporter / emitter
- exact-compare core に絞った minimal diff helper
- fixture authoring / elaboration template

を追加し、PoC を「実行し、artifact を保存し、比較し、次の fixture を足してまた回す」入口まで前進させる。

## 2. Inputs consulted

### 基本導線

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/04-mir-core.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`

### current L2 / parser-free PoC / detached exporter chain

- `specs/examples/02-current-l2-ast-fixture-schema.md`
- `specs/examples/03-current-l2-evaluation-state-schema.md`
- `specs/examples/04-current-l2-step-semantics.md`
- `specs/examples/05-current-l2-oracle-api.md`
- `specs/examples/06-current-l2-interpreter-skeleton.md`
- `specs/examples/07-current-l2-host-stub-harness.md`
- `specs/examples/08-current-l2-host-plan-schema.md`
- `specs/examples/09-current-l2-bundle-loader.md`
- `specs/examples/10-current-l2-batch-runner.md`
- `specs/examples/11-current-l2-selection-helper.md`
- `specs/examples/12-current-l2-selection-profiles.md`
- `specs/examples/13-current-l2-profile-catalog.md`
- `specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md`
- `specs/examples/16-current-l2-detached-trace-audit-artifact-schema.md`
- `specs/examples/17-current-l2-detached-exporter-entry-comparison.md`
- `specs/examples/18-current-l2-bundle-first-detached-payload-context-split.md`
- `specs/examples/19-current-l2-host-plan-coverage-failure-placement.md`
- `specs/examples/20-current-l2-host-plan-coverage-failure-bundle-failure-artifact-schema.md`
- `specs/examples/21-current-l2-host-plan-coverage-failure-aggregate-connection.md`
- `specs/examples/22-current-l2-host-plan-coverage-failure-aggregate-histogram-migration.md`

### plan / report / code anchor

- `plan/00-index.md`
- `plan/05-fallback-lease-and-chain-semantics.md`
- `plan/06-surface-notation-status.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/90-source-traceability.md`
- `plan/91-maintenance-rules.md`
- `docs/reports/0078-current-l2-fallback-lease-regression-fixtures.md`
- `docs/reports/0079-current-l2-fallback-semantic-reconciliation-and-compact-syntax.md`
- `docs/reports/0085-plan-repository-memory-externalization.md`
- `docs/reports/0086-review-plan-memory-doc-boundary-consistency.md`
- `docs/reports/0089-review-followup-0088-and-poc-blockers.md`
- `docs/reports/0090-current-l2-trace-audit-vs-host-interface-boundary.md`
- `docs/reports/0091-review-current-l2-boundary-comparison-narrow-scope.md`
- `docs/reports/0092-current-l2-detached-trace-audit-artifact-schema.md`
- `docs/reports/0093-review-current-l2-detached-trace-audit-artifact-schema.md`
- `docs/reports/0094-current-l2-detached-exporter-entry-layer-comparison.md`
- `docs/reports/0095-review-current-l2-detached-exporter-entry-layer-comparison.md`
- `docs/reports/0096-current-l2-bundle-first-detached-payload-context-split.md`
- `docs/reports/0097-review-current-l2-bundle-first-detached-payload-context-split.md`
- `docs/reports/0098-host-plan-coverage-failure-placement.md`
- `docs/reports/0099-review-current-l2-host-plan-coverage-failure-placement.md`
- `docs/reports/0100-host-plan-coverage-failure-bundle-failure-artifact-schema.md`
- `docs/reports/0101-review-host-plan-coverage-failure-bundle-failure-artifact-schema.md`
- `docs/reports/0102-review-host-plan-coverage-failure-aggregate-connection.md`
- `docs/reports/0103-host-plan-coverage-failure-aggregate-connection.md`
- `docs/reports/0104-host-plan-coverage-failure-aggregate-histogram-migration.md`
- `docs/reports/0105-review-host-plan-coverage-failure-aggregate-histogram-migration.md`
- `crates/mir-semantics/src/lib.rs`
- `crates/mir-semantics/src/harness.rs`
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
- `crates/mir-ast/tests/fixtures/current-l2/`

## 3. Actions taken

1. detached exporter chain の current docs-only judgment を 16..22 から再読し、1 本の consolidation 文書へまとめた。
2. detached export loop の next narrow step を `specs/examples/23-current-l2-detached-export-loop-consolidation.md` として追加した。
3. `run_bundle` / `BundleRunReport` を起点に 1 bundle artifact を出せる non-production example binary を追加した。
4. payload core の exact-compare に絞った minimal diff helper を `scripts/` に追加した。
5. fixture authoring / elaboration を完全解消ではなく template 化するため、`plan/15-current-l2-fixture-authoring-template.md` を追加した。
6. `Documentation.md`、`specs/00-document-map.md`、`plan/00`、`plan/07`、`plan/09`、`plan/11`、`plan/12`、`plan/90` を mirror 更新した。
7. E3 / E6 で tiny exporter と diff helper の smoke evidence を取った。
8. final validation と reviewer はこのあと記録を追記する。

## 4. Files changed

### 追加

- `specs/examples/23-current-l2-detached-export-loop-consolidation.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `crates/mir-semantics/examples/current_l2_emit_detached_bundle.rs`
- `scripts/current_l2_diff_detached_artifacts.py`

### 更新

- `Documentation.md`
- `specs/00-document-map.md`
- `plan/00-index.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`

## 5. Commands run and exact outputs

- `git status --short --branch`
  - `## main...origin/main`
- `mkdir -p crates/mir-semantics/examples`
  - 無出力
- `chmod +x scripts/current_l2_diff_detached_artifacts.py && cargo fmt`
  - 無出力
- `cargo test -p mir-semantics`
  - `test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s`
  - `test result: ok. 33 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s`
  - `test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s`
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 107 numbered report(s).`
- `git diff --check`
  - 無出力
- smoke evidence
  - `cargo run -q -p mir-semantics --example current_l2_emit_detached_bundle -- crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json --output /tmp/.../e3.json`
  - `cargo run -q -p mir-semantics --example current_l2_emit_detached_bundle -- crates/mir-ast/tests/fixtures/current-l2/e6-write-after-expiry.json --output /tmp/.../e6.json`
  - `python3 scripts/current_l2_diff_detached_artifacts.py /tmp/.../e3.json /tmp/.../e3.json`
    - `payload_core: exact-compare core matched`
  - `python3 scripts/current_l2_diff_detached_artifacts.py /tmp/.../e3.json /tmp/.../e6.json`
    - `payload_core.terminal_outcome: left="success" right="Reject"`
    - `payload_core.event_kinds: left=["perform-success"] right=["Reject"]`
    - `payload_core.non_admissible_metadata: left=[{"option_ref": "owner_writer", "subreason": "admit-miss"}] right=[{"option_ref": "writer", "subreason": "lease-expired"}]`
    - `payload_core.narrative_explanations: left=[] right=["readonly remains a request/capability mismatch narrative explanation"]`
- reviewer wait
  - `wait_agent(..., timeout_ms=180000)` → timeout
  - `wait_agent(..., timeout_ms=180000)` → timeout

## 6. Evidence / findings

- task-start dirty state は clean だった。
  - `git status --short --branch` は `## main...origin/main`
- detached exporter chain 16..22 は、`specs/examples/23-current-l2-detached-export-loop-consolidation.md` へ集約しても meaning drift を起こさない粒度まで揃っていた。
- tiny emitter は helper stack 本体へ入れず、`examples/` 配置の non-production helper に留めたため、library public API を増やしていない。
- emitted artifact は current docs-only split と一致する。
  - `payload_core`
  - `bundle_context`
  - `detached_noncore`
  - `must_explain` を含まない
  - success payload core に `host_plan_coverage_failure` を含めない
- diff helper は payload core の exact-compare に絞られており、`bundle_context` / `detached_noncore` は reference-only、`must_explain` は compare 対象外である。
- E3 self-compare は match、E3 vs E6 は outcome / events / formal metadata / short narrative の違いを抽出できた。
- reviewer completion は得られなかったため、task policy に従って local review fallback を採った。
- `plan/` は更新済みであり、`plan/ 更新不要` ではない。

## 7. Changes in understanding

- detached exporter chain 16..22 は、`specs/examples/23-current-l2-detached-export-loop-consolidation.md` へ集約してよい段階まで揃っていた。
- tiny emitter は helper stack 本体へ入れるより、bundle-first public behavior を再利用する non-production example binary として切る方が boundary を壊しにくい。
- diff helper は test oracle へ寄せるより repo-level script として payload core compare に絞る方が、`must_explain` を human-facing に残したまま loop を回しやすい。
- `fixture authoring / elaboration` は依然 bottleneck だが、最低限の template を先に置くことで detached export loop と authoring task の境界をかなり明示できる。

## 8. Open questions

- actual exporter API
- detached artifact 保存先と path policy
- aggregate typed field の actual implementation timing
- richer host interface の typed carrier 化
- final parser syntax
- multi-request scheduler
- `Approximate` / `Compensate`
- static analysis / theorem prover 側との boundary

## 8.1 Commit hashes

- 仕様本文 / helper / plan mirror commit
  - `6c4cb8a` `detached export loop の最小 helper を追加する`
- この report 自身の commit hash は self-reference の都合で本文へ固定しない。

## 9. Suggested next prompt

current L2 parser-free PoC 基盤を前提に、`current_l2_emit_detached_bundle` と `current_l2_diff_detached_artifacts.py` を足した detached export loop の smoke evidence を踏まえて、actual exporter API をまだ固定せずに、**artifact 保存先 / path policy と aggregate export の object-map vs row-list cut** を narrow scope で比較してください。`fixture authoring / elaboration` は独立 bottleneck のままにし、richer host interface には進まないでください。
