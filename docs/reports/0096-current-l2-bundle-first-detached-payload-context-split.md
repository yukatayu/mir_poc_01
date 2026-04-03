# Report 0096 — current l2 bundle first detached payload context split

- Date: 2026-04-03T03:20:00.981909Z
- Author / agent: Codex
- Scope: `run_bundle` / `BundleRunReport` を first exporter entry とする detached artifact の docs-only payload/context split
- Decision levels touched: L2 docs-only boundary refinement

## 1. Objective

current L2 parser-free PoC 基盤を前提に、bundle-first detached exporter で

- 何を payload core に置くか
- 何を bundle-level context に置くか
- 何を detached non-core に置くか
- 何を human-facing explanation に残すか

を narrow scope で整理する。

特に、

- `fixture_id`
- `fixture_path`
- `host_plan_path`
- `runtime_requirement`
- `host_plan_coverage_failure`

の placement を source-backed に比較する。

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
- `specs/examples/07-current-l2-host-stub-harness.md`
- `specs/examples/08-current-l2-host-plan-schema.md`
- `specs/examples/09-current-l2-bundle-loader.md`
- `specs/examples/10-current-l2-batch-runner.md`
- `specs/examples/16-current-l2-detached-trace-audit-artifact-schema.md`
- `specs/examples/17-current-l2-detached-exporter-entry-comparison.md`
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
- `crates/mir-semantics/src/lib.rs`
- `crates/mir-semantics/src/harness.rs`
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
- `crates/mir-ast/tests/fixtures/current-l2/`

## 3. Actions taken

1. canonical docs と code anchor を読み直し、`RunReport` / `FixtureBundle` / `BundleRunReport` / `BatchRunSummary` の current boundary を再確認した。
2. `fixture_id` / `fixture_path` / `host_plan_path` / `runtime_requirement` を
   - top-level
   - `detached_noncore`
   - `bundle_context`
   のどこに置くべきか比較した。
3. `host_plan_coverage_failure` を
   - payload core
   - detached non-core
   - aggregate-only
   の 3 案で比較した。
4. docs-only 正本として `specs/examples/18-current-l2-bundle-first-detached-payload-context-split.md` を追加した。
5. `Documentation.md`、`specs/00-document-map.md`、`plan/07`、`plan/09`、`plan/11`、`plan/12`、`plan/90` に最小 mirror を入れた。
6. task-start dirty state は次の未追跡だけであることを確認した。
   - `diff_investigation_01.md`
   - `旧資料_参考_ChatGPT_01_69c5e3f6/`
   - `旧資料_参考_ChatGPT_02_Mirrorea_2025/`
   - `docs/reports/0096-current-l2-bundle-first-detached-payload-context-split.md`（本 task 開始時の report template）

## 4. Files changed

- `specs/examples/18-current-l2-bundle-first-detached-payload-context-split.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `docs/reports/0096-current-l2-bundle-first-detached-payload-context-split.md`

## 5. Commands run and exact outputs

### `git status --short --branch`

```text
## main...origin/main [ahead 4]
?? diff_investigation_01.md
?? docs/reports/0096-current-l2-bundle-first-detached-payload-context-split.md
?? "旧資料_参考_ChatGPT_01_69c5e3f6/"
?? "旧資料_参考_ChatGPT_02_Mirrorea_2025/"
```

### `python3 scripts/new_report.py --slug current-l2-bundle-first-detached-payload-context-split`

```text
/home/yukatayu/dev/mir_poc_01/docs/reports/0096-current-l2-bundle-first-detached-payload-context-split.md
/home/yukatayu/dev/mir_poc_01/scripts/new_report.py:48: DeprecationWarning: datetime.datetime.utcnow() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.now(datetime.UTC).
  text = text.replace("- Date:", f"- Date: {datetime.utcnow().isoformat()}Z")
```

### `git diff --stat -- Documentation.md specs/00-document-map.md specs/examples/18-current-l2-bundle-first-detached-payload-context-split.md plan/07-parser-free-poc-stack.md plan/09-helper-stack-and-responsibility-map.md plan/11-roadmap-near-term.md plan/12-open-problems-and-risks.md plan/90-source-traceability.md`

```text
 Documentation.md                               |  5 ++--
 plan/07-parser-free-poc-stack.md               | 32 ++++++++++++++++++++++++++
 plan/09-helper-stack-and-responsibility-map.md | 20 ++++++++++++++++
 plan/11-roadmap-near-term.md                   |  2 ++
 plan/12-open-problems-and-risks.md             |  6 +++++
 plan/90-source-traceability.md                 |  8 +++----
 specs/00-document-map.md                       |  6 +++++
 7 files changed, 73 insertions(+), 6 deletions(-)
```

### `python3 scripts/validate_docs.py`

```text
Documentation scaffold looks complete.
Found 97 numbered report(s).
```

### `git diff --check`

```text
<no output>
```

### `cargo test -p mir-semantics`

```text
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 33 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

### reviewer completion

```text
Finding 1:
- plan/90 の parser-free PoC stack report-chain section に report 0096 が抜けている

No other findings.
The substantive judgment is source-backed and remains narrow.
```

## 6. Evidence / findings

### 6.1 payload core

bundle-first exporter の payload core は `RunReport` 由来のまま保つのが最小である。
具体的には次である。

- `static_verdict`
- `entered_evaluation`
- `terminal_outcome`
- `event_kinds`
- formal `non_admissible_metadata`
- short `narrative_explanations`

`steps_executed` は `RunReport` に入っているが、current L2 では detached non-core に留める。

### 6.2 `bundle_context`

`fixture_id` / `fixture_path` / `host_plan_path` / `runtime_requirement` は、top-level でも detached non-core でもなく、`bundle_context` に独立して置くのが最も自然である。

理由は次の通りである。

- `FixtureBundle` が現在すでに持っている bundle identity / sidecar binding と対応づけやすい
- payload core と context の cut が明瞭になる
- `steps_executed` や auxiliary counters のような non-core 補助情報と混ざらない

### 6.3 detached non-core

current L2 の bundle-first artifact で detached non-core に置くのは次である。

- `steps_executed`
- auxiliary counters
- coverage explanation
- host-plan explanation

これは後比較には便利だが、exact-compare core に上げると helper refactor に過敏になるためである。

### 6.4 `host_plan_coverage_failure`

`host_plan_coverage_failure` は current code では

- `RunReport` にも
- `BundleRunReport` にも

入っていない。`batch_summary_from_discovery` で error text から分類され、`BatchBundleOutcome::Failed { host_plan_coverage_failure: true }` と `BatchRunSummary.host_plan_coverage_failures` として materialize される。

そのため、

- payload core には置かない
- detached non-core にも無理に入れない
- **aggregate-only に残す**

のが current helper stack と最も整合する。

### 6.5 human-facing explanation

引き続き detached artifact に上げないものは次である。

- `must_explain`
- long-form audit
- why-this-is-good/bad の prose
- report に残す比較理由

### 6.6 `fixture authoring / elaboration`

今回の split 整理でも、`fixture authoring / elaboration` は independent bottleneck のまま残る。
detached exporter の payload/context split を切っても、この bottleneck 自体は消えない。

### 6.7 reviewer finding

reviewer finding は 1 件だけで、内容は `plan/90-source-traceability.md` の parser-free PoC stack report chain に `0096` が抜けていたことだった。
この漏れは main task 内で修正済みであり、設計判断そのものへの finding はなかった。

## 7. Changes in understanding

- detached artifact の bundle-first split では、payload core / detached non-core の二分だけでは足りず、`bundle_context` を独立 section にした方が current helper boundary と素直に対応することが明確になった。
- `host_plan_coverage_failure` は detached artifact 一般では batch-layer core と読めても、bundle-first artifact の内部にはまだ自然に入らない。
- したがって「bundle-first exporter を始める」と「coverage failure を bundle payload に落とす」は別 task に分ける方がきれいである。

## 8. Open questions

- actual exporter API の形
- detached artifact 保存先と path policy
- bundle-level failure artifact を separate に切るかどうか
- `host_plan_coverage_failure` の typed bundle-level carrier をいつ導入するか
- `BatchRunSummary` aggregate export の閾値
- `fixture authoring / elaboration` bottleneck とどう並走するか

仕様本文コミット hash:

- `13cc1f0` `bundle-first detached artifact の split を整理する`

report 自身の commit hash は self-reference の都合でこの本文には固定しない。

## 9. Suggested next prompt

current L2 parser-free PoC を前提に、bundle-first detached artifact の docs-only split を踏まえて、aggregate export 側に残した `host_plan_coverage_failure` を将来 typed carrier に昇格させるならどの layer で切るのが自然かを narrow scope で比較してください。`run_bundle` payload 自体は変えず、`BatchRunSummary` と bundle failure artifact の境界だけを扱ってください。
