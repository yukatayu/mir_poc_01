# Report 0103 — host plan coverage failure aggregate connection

- Date: 2026-04-03
- Author / agent: Codex
- Scope: current L2 parser-free PoC における typed bundle failure artifact と `BatchRunSummary` aggregate export の接続面 comparison
- Decision levels touched: L2

## 1. Objective

current L2 parser-free PoC 基盤と 0090 / 0091 / 0092 / 0093 / 0094 / 0095 / 0096 / 0097 / 0098 / 0099 / 0100 / 0101 を前提に、bundle failure artifact 側の `failure.failure_kind` discriminator-only schema を `BatchRunSummary` aggregate export がどこまで typed に吸うべきかを docs-only で比較する。

production exporter 実装、runtime semantics、parser grammar、failure family、machine-check policy は変更しない。

## 2. Scope and assumptions

- current detached artifact の settled judgment は次の通りとした。
  - current detached artifact では `host_plan_coverage_failure` を aggregate-only に残す。
  - future typed placement の最小核は bundle failure artifact 側の `failure.failure_kind` discriminator-only。
- `fixture authoring / elaboration` は独立 bottleneck として残す。
- richer host interface には進まない。
- reviewer は最後に 1 回だけ起動し、completion まで待った。

## 3. Documents consulted

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
- `crates/mir-semantics/src/lib.rs`
- `crates/mir-semantics/src/harness.rs`
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`

## 4. Actions taken

1. task-start dirty state を固定した。
2. `RunReport` / `BundleRunReport` / `BatchBundleOutcome::Failed` / `BatchRunSummary` / tests の current code anchor を再確認した。
3. aggregate connection の 3 案を比較した。
   - current counter / list 維持
   - typed histogram / kind count
   - bundle failure summary の薄い再掲
4. docs-only 正本として `specs/examples/21-current-l2-host-plan-coverage-failure-aggregate-connection.md` を追加した。
5. `Documentation.md`、`specs/00-document-map.md`、`specs/examples/20-current-l2-host-plan-coverage-failure-bundle-failure-artifact-schema.md`、`plan/07`、`plan/09`、`plan/11`、`plan/12`、`plan/90` に mirror を反映した。
6. reviewer を 1 回だけ起動し、completion まで待った。review result は `docs/reports/0102-review-host-plan-coverage-failure-aggregate-connection.md` に保存された。
7. reviewer finding 3 件を反映した。
   - harness error wording test も current anchor であることを `specs/examples/21` に追記
   - `specs/00-document-map.md` の spec 21 mirror wording を aggregate connection comparison に修正
   - `plan/90` から未作成 report 番号の先走りを外した
8. `python3 scripts/validate_docs.py`、`git diff --check`、`cargo test -p mir-semantics` を通した。
9. main docs/spec/plan 変更と review artifact を commit `6be11cb` `host plan coverage failure の aggregate 接続を整理する` にまとめた。
10. reviewer finding の follow-up と whitespace hygiene を commit `59dcb34` `aggregate 接続比較の review 補正を反映する` にまとめた。

## 5. Files changed

- `Documentation.md`
- `specs/00-document-map.md`
- `specs/examples/20-current-l2-host-plan-coverage-failure-bundle-failure-artifact-schema.md`
- `specs/examples/21-current-l2-host-plan-coverage-failure-aggregate-connection.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `docs/reports/0102-review-host-plan-coverage-failure-aggregate-connection.md`
- `docs/reports/0103-host-plan-coverage-failure-aggregate-connection.md`

## 6. Commands run and exact outputs

- `git status --short --branch`
  - task start:
    - `## main...origin/main [ahead 7]`
    - `?? diff_investigation_01.md`
    - `?? 旧資料_参考_ChatGPT_01_69c5e3f6/`
    - `?? 旧資料_参考_ChatGPT_02_Mirrorea_2025/`
- `python3 scripts/validate_docs.py`

```text
Documentation scaffold looks complete.
Found 102 numbered report(s).
```

- `git diff --check`

```text
<no output>
```

- `cargo test -p mir-semantics`

```text
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 33 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

- `git commit --no-gpg-sign -m "host plan coverage failure の aggregate 接続を整理する"`

```text
[main 6be11cb] host plan coverage failure の aggregate 接続を整理する
 10 files changed, 380 insertions(+), 7 deletions(-)
 create mode 100644 docs/reports/0102-review-host-plan-coverage-failure-aggregate-connection.md
 create mode 100644 specs/examples/21-current-l2-host-plan-coverage-failure-aggregate-connection.md
```

- `git commit --no-gpg-sign -m "aggregate 接続比較の review 補正を反映する"`

```text
[main 59dcb34] aggregate 接続比較の review 補正を反映する
 2 files changed, 6 insertions(+), 6 deletions(-)
```

## 7. Evidence / outputs / test results

### 7.1 code anchor

- `RunReport` は payload core だけを持ち、`host_plan_coverage_failure` を持たない。
- `BundleRunReport` は `report: RunReport` の thin wrapper である。
- `BatchBundleOutcome::Failed { host_plan_coverage_failure: bool }` は current per-bundle failure bit である。
- `BatchRunSummary.host_plan_coverage_failures` は `BundleExecutionFailure` の list である。
- `batch_summary_from_discovery()` が error text から coverage failure を分類している。
- tests は summary / batch report だけでなく harness error wording も固定している。

### 7.2 comparison judgment

- aggregate 側に **typed 集約を持たせるなら最小は histogram / kind count** である。
- current counter / list 維持は implementation timing としては妥当な次点案である。
- bundle failure summary の薄い再掲は採らない。

理由:

- batch summary の coarse 役割を壊しにくい。
- bundle failure artifact と責務競合しない。
- exact-compare payload core を汚さない。
- `bundle_context`、detached non-core explanation、`must_explain` を aggregate 側に持ち込まずに済む。

### 7.3 artifact / aggregate の cut

- bundle artifact に残すもの
  - `failure.failure_kind`
  - `bundle_context`
  - detached non-core explanation
  - `must_explain`
- aggregate 側に出してよい最小情報
  - `failure_kind`
  - `count`
- aggregate 側に出さないもの
  - bundle failure summary の再掲
  - `bundle_context`
  - detached non-core explanation
  - `must_explain`

### 7.4 `fixture authoring / elaboration`

- この task と独立 bottleneck のままである。
- detached aggregate connection を narrow に切っても authoring cost 自体は減らない。
- ただし typed histogram は多件比較の coarse view としては有益である。

## 8. What changed in understanding

- bundle failure artifact 側の typed core を aggregate が吸うとしても、aggregate は count 集約より厚くしない方が current helper boundary と整合することが明確になった。
- typed failure flat list は「typed に見えるが coarse summary ではない」中間形で、最小案としての価値が薄いと整理できた。
- harness error wording test が current anchor に含まれるため、future typed migration の影響範囲は batch summary だけではないと明確になった。

## 9. Open questions

- actual exporter API
- current list / bool shape と typed histogram の migration timing
- detached artifact 保存先と path policy
- richer host interface の typed carrier 化
- multi-request scheduler
- `Approximate` / `Compensate`
- static analysis / theorem prover 側との boundary

仕様本文コミット hash:

- `6be11cb` `host plan coverage failure の aggregate 接続を整理する`
- `59dcb34` `aggregate 接続比較の review 補正を反映する`

この report 自身の commit hash は self-reference の都合で本文には固定しない。

## 10. Suggested next prompt

`specs/examples/21-current-l2-host-plan-coverage-failure-aggregate-connection.md` を前提に、current L2 parser-free PoC で bundle failure artifact の `failure.failure_kind` histogram を aggregate export 側へ入れるなら、その field 名と migration cut を docs-only で narrow に比較してください。current list / bool shape を置き換えるか併存させるか、そして batch summary の coarse role をどう保つかに絞ってください。
