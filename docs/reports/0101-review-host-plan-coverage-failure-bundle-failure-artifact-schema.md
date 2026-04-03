# Report 0101 — review host plan coverage failure bundle failure artifact schema

- Date: 2026-04-03
- Author / agent: Codex reviewer pass
- Scope: `host_plan_coverage_failure` typed bundle failure artifact schema の docs-only refinement に対する narrow review
- Decision levels touched: L2

## 1. Objective

`specs/examples/20-current-l2-host-plan-coverage-failure-bundle-failure-artifact-schema.md` とその mirror / report が、current code anchor と helper boundary を壊していないかを narrow scope で確認する。

## 2. Scope and assumptions

- review 対象は docs-only 変更に限定する。
- runtime semantics、parser grammar、failure family、machine-check policy は変えない前提で見る。
- 特に次の 3 点に絞る。
  - `failure.failure_kind` discriminator-only が最小 schema として妥当か
  - `bundle_context` / detached non-core / human-facing explanation の cut が保たれているか
  - current code anchor (`RunReport`, `BundleRunReport`, `BatchBundleOutcome::Failed`, `BatchRunSummary`) と矛盾しないか

## 3. Documents consulted

- `specs/examples/19-current-l2-host-plan-coverage-failure-placement.md`
- `specs/examples/20-current-l2-host-plan-coverage-failure-bundle-failure-artifact-schema.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/12-open-problems-and-risks.md`
- `docs/reports/0100-host-plan-coverage-failure-bundle-failure-artifact-schema.md`
- `crates/mir-semantics/src/lib.rs`
- `crates/mir-semantics/src/harness.rs`

## 4. Actions taken

1. reviewer を 1 回だけ回し、completion まで待った。
2. reviewer の返答を current workspace の source と突き合わせた。
3. local でも `RunReport` / `BundleRunReport` / `BatchBundleOutcome::Failed` / `BatchRunSummary` の code anchor を再確認した。

## 5. Files changed

- `docs/reports/0101-review-host-plan-coverage-failure-bundle-failure-artifact-schema.md`

## 6. Evidence / outputs / test results

### reviewer result

- reviewer からの実質的な finding は無かった。
- 特に否定されなかった判断は次の通り。
  - `host_plan_coverage_failure` は success payload core に上げない
  - future typed carrier の最小核は `failure.failure_kind` の discriminator-only
  - `bundle_context` は artifact 全体の別 section に残す
  - short coverage note は detached non-core に残す

### local confirmation

- `RunReport` は `host_plan_coverage_failure` を持たない。
- `BundleRunReport` は `report: RunReport` だけを持つ。
- `BatchBundleOutcome::Failed` は `host_plan_coverage_failure: bool` を持つ。
- `BatchRunSummary.host_plan_coverage_failures` は `BundleExecutionFailure` の配列である。

したがって、今回の schema refinement は current code の materialization point と矛盾しない。

## 7. What changed in understanding

- reviewer を通した後も、schema の最小核は `failure_kind` だけで十分という判断は崩れなかった。
- この task で追加の mirror 修正を広げる必要は見えなかった。

## 8. Open questions

- actual exporter API
- detached artifact 保存先と path policy
- `BatchRunSummary` aggregate export との接続時期
- string detection を typed detection に置き換える入口
- richer host interface の typed carrier 化との接続

## 9. Suggested next prompt

`specs/examples/20-current-l2-host-plan-coverage-failure-bundle-failure-artifact-schema.md` を前提に、bundle failure artifact 側の `failure_kind` discriminator-only schema と `BatchRunSummary` aggregate export の接続面を docs-only で比較し、aggregate 側にどこまで typed 集約を持たせるべきかを narrow scope で整理してください。
