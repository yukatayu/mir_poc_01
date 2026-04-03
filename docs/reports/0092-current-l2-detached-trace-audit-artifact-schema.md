# Report 0092 — current l2 detached trace audit artifact schema

- Date: 2026-04-03T01:47:07.317749Z
- Author / agent: Codex
- Scope: `specs/examples/16-current-l2-detached-trace-audit-artifact-schema.md`、`Documentation.md`、`specs/00-document-map.md`、`plan/07-parser-free-poc-stack.md`、`plan/11-roadmap-near-term.md`、`plan/12-open-problems-and-risks.md`、`plan/90-source-traceability.md`
- Decision levels touched: L2 docs-only boundary整理

## 1. Objective

current L2 parser-free PoC 基盤を前提に、detached trace / audit serialization の最小 artifact schema を docs-only で切り出し、`RunReport` / `BundleRunReport` / `BatchRunSummary` のどの field だけを exact-compare core として外へ出すべきかを narrow scope で整理する。

重要な前提は次のとおりである。

- これは repo 全体の唯一の次 bottleneck を確定する task ではない
- `fixture authoring / elaboration` は依然として独立 bottleneck である
- detached trace / audit と richer host interface の 2 項目を比べるなら detached を先に切る、という prior comparison を 1 段具体化する
- runtime semantics、parser grammar、failure family、machine-check surface は変えない

## 2. Inputs consulted

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
- `plan/01-status-at-a-glance.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `docs/reports/0054-current-l2-parser-free-interpreter-skeleton.md`
- `docs/reports/0056-current-l2-host-stub-harness.md`
- `docs/reports/0059-current-l2-host-plan-sidecar-loader.md`
- `docs/reports/0060-current-l2-bundle-loader.md`
- `docs/reports/0062-current-l2-batch-runner.md`
- `docs/reports/0077-current-l2-helper-layer-responsibility-alignment.md`
- `docs/reports/0085-plan-repository-memory-externalization.md`
- `docs/reports/0086-review-plan-memory-doc-boundary-consistency.md`
- `docs/reports/0088-current-l2-representative-prose-drift-check.md`
- `docs/reports/0089-review-followup-0088-and-poc-blockers.md`
- `docs/reports/0090-current-l2-trace-audit-vs-host-interface-boundary.md`
- `docs/reports/0091-review-current-l2-boundary-comparison-narrow-scope.md`
- `crates/mir-semantics/src/lib.rs`
- `crates/mir-semantics/src/harness.rs`
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`

## 3. Actions taken

1. AGENTS の順序に従って `README.md`、`Documentation.md`、`specs/00`〜`03`、`specs/09` を先に確認した。
2. current L2 parser-free PoC stack と detached trace / audit comparison に関係する spec / plan / report / code anchor を読んだ。
3. `ExpectedTraceAudit`、`TraceAuditSink`、`RunReport`、`BundleRunReport`、`BatchRunSummary` の現在 carrier を照合した。
4. field 候補を
   - exact-compare core
   - detached artifact には入れてよいが machine-check core ではない
   - human-facing explanation に残す
   の 3 群に切り分けた。
5. detached run / bundle / batch の docs-only schema sketch を新規 spec として追加した。
6. `Documentation.md`、`specs/00-document-map.md`、`plan/07`、`plan/11`、`plan/12`、`plan/90` に最小 mirror を入れた。

## 4. Files changed

- 新規: `docs/reports/0092-current-l2-detached-trace-audit-artifact-schema.md`
- 新規: `docs/reports/0093-review-current-l2-detached-trace-audit-artifact-schema.md`
- 新規: `specs/examples/16-current-l2-detached-trace-audit-artifact-schema.md`
- 更新: `Documentation.md`
- 更新: `specs/00-document-map.md`
- 更新: `plan/07-parser-free-poc-stack.md`
- 更新: `plan/11-roadmap-near-term.md`
- 更新: `plan/12-open-problems-and-risks.md`
- 更新: `plan/90-source-traceability.md`
- 更新: `docs/reports/0092-current-l2-detached-trace-audit-artifact-schema.md`

## 5. Commands run and exact outputs

```bash
python3 scripts/new_report.py --slug current-l2-detached-trace-audit-artifact-schema
python3 scripts/new_report.py --slug review-current-l2-detached-trace-audit-artifact-schema
git status --short --branch
python3 scripts/validate_docs.py
git diff --check
cargo test -p mir-semantics
git -c commit.gpgsign=false commit -m "detached trace artifact の docs-only schema を整理する"
```

### Exact outputs

- `python3 scripts/new_report.py --slug current-l2-detached-trace-audit-artifact-schema`

```text
/home/yukatayu/dev/mir_poc_01/docs/reports/0092-current-l2-detached-trace-audit-artifact-schema.md
/home/yukatayu/dev/mir_poc_01/scripts/new_report.py:48: DeprecationWarning: datetime.datetime.utcnow() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.now(datetime.UTC).
  text = text.replace("- Date:", f"- Date: {datetime.utcnow().isoformat()}Z")
```

- task-start dirty state の `git status --short --branch`

```text
## main...origin/main
?? diff_investigation_01.md
?? "旧資料_参考_ChatGPT_01_69c5e3f6/"
?? "旧資料_参考_ChatGPT_02_Mirrorea_2025/"
```

- `python3 scripts/new_report.py --slug review-current-l2-detached-trace-audit-artifact-schema`

```text
/home/yukatayu/dev/mir_poc_01/docs/reports/0093-review-current-l2-detached-trace-audit-artifact-schema.md
/home/yukatayu/dev/mir_poc_01/scripts/new_report.py:48: DeprecationWarning: datetime.datetime.utcnow() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.now(datetime.UTC).
  text = text.replace("- Date:", f"- Date: {datetime.utcnow().isoformat()}Z")
```

- `python3 scripts/validate_docs.py`

```text
Documentation scaffold looks complete.
Found 93 numbered report(s).
```

- `git diff --check`

```text
<no output>
```

- `cargo test -p mir-semantics`

```text
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 33 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

- `git -c commit.gpgsign=false commit -m "detached trace artifact の docs-only schema を整理する"`

```text
[main cb52744] detached trace artifact の docs-only schema を整理する
 7 files changed, 324 insertions(+), 11 deletions(-)
 create mode 100644 specs/examples/16-current-l2-detached-trace-audit-artifact-schema.md
```

Verification:

- docs validate は pass
- `git diff --check` は clean
- `cargo test -p mir-semantics` は pass

## 6. Evidence / findings

### 6.1 current machine-check surface の正本

`RunReport` は `static_verdict`、`entered_evaluation`、`terminal_outcome`、`trace_audit_sink`、`steps_executed` を持つ。  
`TraceAuditSink` は `events`、formal `non_admissible_metadata`、short `narrative_explanations` を持つ。  
`ExpectedTraceAudit` は fixture 側 expectation としてこれらに加えて `must_explain` を持つ。  
bundle helper はすでに次を exact compare している。

- `static_verdict`
- `entered_evaluation`
- `terminal_outcome`
- `event_kinds`
- formal `non_admissible_metadata`
- short `narrative_explanations`

一方で `must_explain` は exact compare していない。

### 6.2 detached artifact の exact-compare core

今回の判断では、次を detached artifact の exact-compare core に置く。

- `static_verdict`
- `entered_evaluation`
- `terminal_outcome`
- `event_kinds`
- formal `non_admissible_metadata`
- short `narrative_explanations`
- batch-layer の `host_plan_coverage_failure`

`host_plan_coverage_failure` は trace event ではないが、batch / profile 実行を後比較するときに uncovered oracle call を通常 mismatch と分ける最小 operational signal になるため、batch-layer core として detached artifact に出してよい。

### 6.3 detached artifact には入れてよいが core にしないもの

次は detached artifact には残してよいが、exact-compare core にはしない。

- `steps_executed`
- coverage explanation
- host-plan explanation
- auxiliary counters / summaries

特に `steps_executed` は後比較には有益だが、current L2 では step granularity 自体が final runtime contract ではないため、core に上げると interpreter refactor に過敏になる。

### 6.4 human-facing explanation に残すもの

次は detached artifact core に上げず、人間向け説明 obligation として残す。

- `must_explain`
- long-form audit explanation
- static verdict reason の長文
- why-this-is-good/bad の比較 prose

これは current L2 の machine-check と human-facing explanation の分離を保つためである。

### 6.5 richer host interface を後段に残す理由

今回の task は detached artifact 側だけを 1 段具体化する narrow scope であり、richer host interface は次の論点を後段に残す。

- uncovered call detection の typed 化
- preflight coverage analysis
- coverage explanation の置き場
- `PredicateOracle` / `EffectOracle` carrier をどこまで増やすか

これらは detached artifact docs-only schema を壊さず後から narrow に足せる。

### 6.6 `fixture authoring / elaboration` の位置づけ

今回の判断は repo 全体の唯一の次 bottleneck を確定するものではない。  
`fixture authoring / elaboration` は依然として独立 bottleneck であり、detached trace / audit serialization と richer host interface の 2 項目を比べたときに మాత్రమే detached を先に切る。

### 6.7 reviewer finding と反映

reviewer は 2 点を指摘した。

1. bundle artifact sketch で `fixture_path` を top-level に置くと、detached non-core に分類した path 情報との切り分けがぼやける
2. `plan/07` の docs-only boundary summary が `host-plan explanation` を落としており、spec との mirror drift になっている

このため、次を反映した。

- `specs/examples/16-current-l2-detached-trace-audit-artifact-schema.md`
  - bundle artifact sketch の `fixture_path` / `host_plan_path` を `detached_noncore` へ移動
- `plan/07-parser-free-poc-stack.md`
  - docs-only boundary summary に `host-plan explanation` を追記

その他について reviewer は no findings だった。特に、

- `must_explain` を machine-check に上げていない点
- `host_plan_coverage_failure` を batch-layer core としつつ typed carrier は未決に留めている点
- `steps_executed` を detached non-core に置いた点

は current implementation / plan mirror と整合していると確認できた。

### 6.8 commit boundary

仕様本文 / plan 本文 / Documentation の commit hash は `cb52744` である。  
report 自身の commit hash は self-reference の都合で本文に固定しない。

## 7. Changes in understanding

- detached trace / audit serialization は、production schema を決めなくても docs-only minimal boundary までは切れると分かった。
- `steps_executed` は useful だが semantic core ではなく、detached non-core に置くのが current L2 と整合する。
- `host_plan_coverage_failure` は batch-layer core に置けるが、typed carrier としての固定は richer host interface task に残すのが安全である。
- `must_explain` を human-facing explanation に残す current policy は、そのまま detached artifact boundary にも適用できる。
- `plan/` は更新が必要だった。docs-only schema ができたので、`plan/11` と `plan/12` の detached trace / audit 項目は一段具体化された。

## 8. Open questions

- detached artifact の production schema version をどう切るか。
- 保存パス規約や repo 外 artifact 置き場をどう設けるか。
- `host_plan_coverage_failure` を string detection から typed carrier へいつ移すか。
- `steps_executed` を将来 benchmark smoke 用 field として分けるか。
- static analysis / theorem prover 側へ trace / audit のどこまで渡すか。
- `fixture authoring / elaboration` の bottleneck を detached artifact task とどう並走させるか。

## 9. Suggested next prompt

`specs/examples/16-current-l2-detached-trace-audit-artifact-schema.md` を前提に、current L2 parser-free PoC の detached artifact exporter を production 実装ではなく docs-aligned prototype として narrow に設計し、`RunReport` / `BatchRunSummary` からどの carrier mapping を最初に切るべきかを比較してください。必要なら `experiments/` 下に tiny JSON emitter 例を 1 つだけ置き、richer host interface には進まないでください。`
