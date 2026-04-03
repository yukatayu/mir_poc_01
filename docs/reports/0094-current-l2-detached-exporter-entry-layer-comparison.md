# Report 0094 — current l2 detached exporter entry layer comparison

- Date: 2026-04-03T02:25:56.156232Z
- Author / agent: Codex
- Scope: current L2 parser-free PoC における detached artifact exporter の entry layer comparison
- Decision levels touched: L2

## 1. Objective

current L2 parser-free PoC 基盤を前提に、detached artifact exporter を narrow に始めるなら

1. `RunReport` 直列 export
2. `BundleRunReport` export
3. `BatchRunSummary` export

のどこから切るのが最小で、比較価値が高く、helper boundary を壊さないかを source-backed に比較する。

production exporter 実装、production schema version、richer host interface は固定しない。

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
- `specs/examples/06-current-l2-interpreter-skeleton.md`
- `specs/examples/07-current-l2-host-stub-harness.md`
- `specs/examples/08-current-l2-host-plan-schema.md`
- `specs/examples/09-current-l2-bundle-loader.md`
- `specs/examples/10-current-l2-batch-runner.md`
- `specs/examples/16-current-l2-detached-trace-audit-artifact-schema.md`
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
- `crates/mir-semantics/src/lib.rs`
- `crates/mir-semantics/src/harness.rs`
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`

## 3. Actions taken

1. current docs と code anchor から detached artifact の current machine-check surface を再確認した。
2. `RunReport`、`BundleRunReport`、`BatchRunSummary` の責務と carrier 粒度を比較した。
3. detached artifact の payload core と first exporter entry boundary を分けて判断する方針で整理した。
4. `specs/examples/17-current-l2-detached-exporter-entry-comparison.md` を新設し、comparison を docs-only で固定した。
5. `Documentation.md`、`specs/00-document-map.md`、`plan/07`、`plan/09`、`plan/11`、`plan/12`、`plan/90` に最小 mirror を追加した。
6. task-start dirty state は次の未追跡ファイル / ディレクトリだけであることを確認した。
   - `diff_investigation_01.md`
   - `旧資料_参考_ChatGPT_01_69c5e3f6/`
   - `旧資料_参考_ChatGPT_02_Mirrorea_2025/`
   - `docs/reports/0094-current-l2-detached-exporter-entry-layer-comparison.md`（本 task 開始時の report template）

## 4. Files changed

- `specs/examples/17-current-l2-detached-exporter-entry-comparison.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `docs/reports/0094-current-l2-detached-exporter-entry-layer-comparison.md`

## 5. Commands run and exact outputs

### `git status --short --branch`

```text
## main...origin/main [ahead 2]
?? diff_investigation_01.md
?? docs/reports/0094-current-l2-detached-exporter-entry-layer-comparison.md
?? "旧資料_参考_ChatGPT_01_69c5e3f6/"
?? "旧資料_参考_ChatGPT_02_Mirrorea_2025/"
```

### `git diff --stat -- specs/examples/17-current-l2-detached-exporter-entry-comparison.md Documentation.md specs/00-document-map.md plan/07-parser-free-poc-stack.md plan/09-helper-stack-and-responsibility-map.md plan/11-roadmap-near-term.md plan/12-open-problems-and-risks.md plan/90-source-traceability.md`

```text
 Documentation.md                               |  5 +++--
 plan/07-parser-free-poc-stack.md               | 15 +++++++++++++++
 plan/09-helper-stack-and-responsibility-map.md | 15 +++++++++++++++
 plan/11-roadmap-near-term.md                   | 10 ++++++----
 plan/12-open-problems-and-risks.md             |  6 ++++++
 plan/90-source-traceability.md                 |  6 +++---
 specs/00-document-map.md                       |  6 ++++++
 7 files changed, 54 insertions(+), 9 deletions(-)
```

### `python3 scripts/validate_docs.py`

```text
Documentation scaffold looks complete.
Found 95 numbered report(s).
```

### `git diff --check`

```text
<no output>
```

### `cargo test -p mir-semantics`

```text
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 33 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

### reviewer completion

```text
Finding 1 (Medium):
- plan/09 に追加した detached-exporter boundary judgment の source traceability が incomplete
- plan/90 の plan/09 row に specs/examples/17 と docs/reports/0094 を追加すべき

No other findings.
The underlying judgment itself is consistent with specs/examples/16, specs/examples/17,
and current helper boundaries in harness.rs.
```

## 6. Evidence / findings

- `RunReport` は exact-compare core と最も近い payload carrier である。
- ただし exporter entry を `RunReport` へ直結すると、fixture path / host-plan path / runtime requirement / coverage failure など bundle context を失いやすい。
- `run_bundle` / `BundleRunReport` は current helper stack における 1 bundle の public behavior boundary であり、payload core と detached non-core context を分けたまま narrow exporter entry を置きやすい。
- `BatchRunSummary` は大量比較には便利だが、初手としては coarse であり、selection / profile / aggregation の論点まで exporter に持ち込みやすい。
- したがって current judgment は次である。
  - payload core: `RunReport`
  - first exporter entry: `run_bundle` / `BundleRunReport`
  - second stage: `BatchRunSummary`
- `fixture authoring / elaboration` は detached exporter task と独立 bottleneck のまま残る。
- reviewer は 1 回で completion し、traceability 1 件以外の finding はなかった。

## 7. Changes in understanding

- detached artifact の comparison では、payload core と exporter entry boundary を同一視しない方が current helper stack に整合することが明確になった。
- `RunReport` が payload core に最も近いことと、`BundleRunReport` が first exporter entry に最も自然であることは両立する。
- `BatchRunSummary` は exporter の初手ではなく、第 2 段階の aggregate export として扱う方が narrow である。

## 8. Open questions

- actual exporter API の形
- detached artifact の保存パス規約
- `host_plan_coverage_failure` の typed carrier 化をどの段階で切るか
- batch / profile / named profile をまとめた aggregate export をどの順で検討するか
- `fixture authoring / elaboration` bottleneck を detached exporter task とどう並走させるか

仕様本文コミット hash:

- `2dc8bf8` `detached exporter の entry 境界を整理する`

report 自身の commit hash は self-reference の都合でこの本文には固定しない。

## 9. Suggested next prompt

current L2 parser-free PoC を前提に、`run_bundle` / `BundleRunReport` を detached exporter の first entry とみなしたとき、artifact payload と bundle context をどう二層分離するかを docs-only で比較し、`host_plan_coverage_failure` を core / non-core のどちらへ置くのが自然かを narrow scope で整理してください。
