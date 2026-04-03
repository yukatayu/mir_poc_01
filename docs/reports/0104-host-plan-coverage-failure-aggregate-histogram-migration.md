# Report 0104 — host plan coverage failure aggregate histogram migration

- Date: 2026-04-03
- Author / agent: Codex
- Scope: current L2 parser-free PoC における `host_plan_coverage_failure` aggregate histogram / kind count の field-name / migration-cut comparison
- Decision levels touched: L2 docs-only boundary refinement

## 1. Objective

current L2 parser-free PoC 基盤と `0090`〜`0103` の detached artifact 比較 chain を前提に、aggregate export 側へ typed histogram / kind count を入れるなら、その field 名、shape、migration cut をどう切るのが最小かを source-backed に比較する。

この task では production exporter 実装、actual exporter API、runtime semantics、parser grammar、failure family は固定しない。

## 2. Scope and assumptions

- `host_plan_coverage_failure` の current code anchor は `BatchBundleOutcome::Failed { host_plan_coverage_failure: bool }` と `BatchRunSummary.host_plan_coverage_failures` にある。
- future typed placement の最小核は bundle failure artifact 側の `failure.failure_kind` discriminator-only である。
- aggregate export が typed bundle failure を吸うとしても、bundle failure summary の薄い再掲は採らない。
- `fixture authoring / elaboration` は依然 independent bottleneck として残る。

## 3. Documents consulted

- `plan/00-index.md`
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
- `specs/examples/07-current-l2-host-stub-harness.md`
- `specs/examples/08-current-l2-host-plan-schema.md`
- `specs/examples/09-current-l2-bundle-loader.md`
- `specs/examples/10-current-l2-batch-runner.md`
- `specs/examples/16-current-l2-detached-trace-audit-artifact-schema.md`
- `specs/examples/17-current-l2-detached-exporter-entry-comparison.md`
- `specs/examples/18-current-l2-bundle-first-detached-payload-context-split.md`
- `specs/examples/19-current-l2-host-plan-coverage-failure-placement.md`
- `specs/examples/20-current-l2-host-plan-coverage-failure-bundle-failure-artifact-schema.md`
- `specs/examples/21-current-l2-host-plan-coverage-failure-aggregate-connection.md`
- `plan/01-status-at-a-glance.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
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
- `crates/mir-semantics/src/lib.rs`
- `crates/mir-semantics/src/harness.rs`
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
- `crates/mir-ast/tests/fixtures/current-l2/`

## 4. Actions taken

1. aggregate export の current coarse role と `host_plan_coverage_failure` の current code anchor を再確認した。
2. field 名候補として `typed_failure_histogram`、`failure_kind_histogram`、`bundle_failure_kind_counts` を比較した。
3. shape 候補として row list と object map を比較した。
4. migration cut として current bool/list 維持、typed field への即時置換、bool/list と typed field の併存を比較した。
5. docs-only 正本として `specs/examples/22-current-l2-host-plan-coverage-failure-aggregate-histogram-migration.md` を追加した。
6. `Documentation.md`、`specs/00-document-map.md`、`plan/07`、`plan/09`、`plan/11`、`plan/12`、`plan/90` に最小 mirror を入れた。
7. reviewer を最後に 1 回だけ回し、completion で返った finding を反映した。

## 5. Evidence / outputs / test results

### Files changed

- 新規: `specs/examples/22-current-l2-host-plan-coverage-failure-aggregate-histogram-migration.md`
- 更新: `Documentation.md`
- 更新: `specs/00-document-map.md`
- 更新: `specs/examples/21-current-l2-host-plan-coverage-failure-aggregate-connection.md`
- 更新: `plan/07-parser-free-poc-stack.md`
- 更新: `plan/09-helper-stack-and-responsibility-map.md`
- 更新: `plan/11-roadmap-near-term.md`
- 更新: `plan/12-open-problems-and-risks.md`
- 更新: `plan/90-source-traceability.md`
- 新規: `docs/reports/0104-host-plan-coverage-failure-aggregate-histogram-migration.md`
- 新規: `docs/reports/0105-review-host-plan-coverage-failure-aggregate-histogram-migration.md`

### task-start dirty state

- `git status --short --branch`

```text
## main...origin/main
```

### Commands run and exact outputs

- `python3 scripts/validate_docs.py`

```text
Documentation scaffold looks complete.
Found 105 numbered report(s).
```

- `git diff --check`

```text
<no output>
```

- `cargo test -p mir-semantics`

```text
Finished `test` profile [unoptimized + debuginfo] target(s) in 0.50s
running 2 tests
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

running 33 tests
test result: ok. 33 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

- `git -c commit.gpgsign=false commit -m "aggregate histogram の命名と migration cut を整理する"`

```text
[main d65ce71] aggregate histogram の命名と migration cut を整理する
 9 files changed, 350 insertions(+), 8 deletions(-)
 create mode 100644 specs/examples/22-current-l2-host-plan-coverage-failure-aggregate-histogram-migration.md
```

### Reviewer completion

reviewer completion は非同期で返り、finding は 2 件だった。

1. `docs/reports/0104-host-plan-coverage-failure-aggregate-histogram-migration.md` が draft のままで、evidence chain が閉じていなかった。
2. `Documents consulted` に `plan/00-index.md` が欠けていた。

この 2 件は反映済みで、semantic inconsistency の指摘はなかった。review の記録は `docs/reports/0105-review-host-plan-coverage-failure-aggregate-histogram-migration.md` に置く。

### Findings

現時点の judgment は次のとおりである。

- aggregate 側の最小 typed 集約 shape は、`failure_kind` / `count` row を持つ `bundle_failure_kind_counts` が最小候補である。
- `bundle_` prefix は current `BatchRunSummary` の `bundle_failures` / `bundle_reports` と整合し、discovery 側との混線を避けやすい。
- docs-only の最小 migration cut は current bool/list anchor と typed count field の additive coexistence である。
- current `host_plan_coverage_failures` list と `BatchBundleOutcome::Failed.host_plan_coverage_failure` bool の即時置換は cut が大きすぎ、docs-only judgment としては premature である。

### 仕様本文 commit hash

- `d65ce71` `aggregate histogram の命名と migration cut を整理する`

report 自身の commit hash は self-reference の都合でこの本文には固定しない。

## 6. What changed in understanding

- aggregate typed 集約の最小 shape が histogram / kind count である、という prior judgmentからさらに進めると、field 名も migration cut も current helper boundary に寄せて決めた方が自然だと分かった。
- current naming family に合わせるなら `bundle_failure_kind_counts` が最も narrow であり、`typed_failure_histogram` や `failure_kind_histogram` より drift source が少ない。
- replacement timing は naming より大きい判断であり、docs-only では additive coexistence までに留めるのが安全である。

## 7. Open questions

- actual exporter API
- current bool/list anchor をいつ完全に置き換えるか
- object map と row list のどちらを implementation で採るか
- detached artifact 保存先と path policy
- richer host interface の typed carrier 化
- `fixture authoring / elaboration` とどう並走させるか

## 8. Suggested next prompt

current L2 parser-free PoC 基盤と `specs/examples/22-current-l2-host-plan-coverage-failure-aggregate-histogram-migration.md` を前提に、`bundle_failure_kind_counts` を actual aggregate exporter field にするなら、row list shape を `BatchRunSummary` / profile summary / named profile summary のどこまで mirror すべきかを docs-only で比較してください。current bool/list anchor はまだ残したまま、implementation には進まないでください。
