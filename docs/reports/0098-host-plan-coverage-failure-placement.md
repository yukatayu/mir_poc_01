# Report 0098 — host plan coverage failure placement

- Date: 2026-04-03T04:23:56.250463Z
- Author / agent: Codex
- Scope: current L2 parser-free PoC 基盤における `host_plan_coverage_failure` の placement boundary comparison
- Decision levels touched: L2

## 1. Objective

current L2 parser-free PoC 基盤と 0090 / 0091 / 0092 / 0093 / 0094 / 0095 / 0096 / 0097 を前提に、`host_plan_coverage_failure` を将来 typed carrier に切り出すならどの layer が自然かを narrow scope で比較する。

この task では production exporter 実装、runtime semantics、parser grammar、failure family は変更しない。

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
- `specs/examples/18-current-l2-bundle-first-detached-payload-context-split.md`
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
- `crates/mir-semantics/src/lib.rs`
- `crates/mir-semantics/src/harness.rs`
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
- `crates/mir-ast/tests/fixtures/current-l2/`

## 3. Actions taken

1. current `RunReport` / `BundleRunReport` / `BatchRunSummary` と `host_plan_coverage_failure` の current code での materialization point を読み直した。
2. 次の 3 案を source-backed に比較した。
   - aggregate-only を維持する
   - bundle failure artifact 側に typed carrier を導入する
   - detached non-core / `bundle_context` に薄く置く
3. 比較結果を新しい companion 文書 `specs/examples/19-current-l2-host-plan-coverage-failure-placement.md` に整理した。
4. `Documentation.md`、`specs/00-document-map.md`、`plan/07`、`plan/09`、`plan/11`、`plan/12`、`plan/90` に current understanding を最小反映した。
5. spec / docs / plan 本体を commit `3934303` にまとめた。
6. reviewer tool がこのセッションでは利用できなかったため、scope を限定した local review と verification を evidence として残す方針に切り替えた。

## 4. Files changed

- `Documentation.md`
- `specs/00-document-map.md`
- `specs/examples/19-current-l2-host-plan-coverage-failure-placement.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `docs/reports/0098-host-plan-coverage-failure-placement.md`

## 5. Commands run and exact outputs

- `git status --short --branch`
  - task start:
    - `## main...origin/main`
    - `?? diff_investigation_01.md`
    - `?? 旧資料_参考_ChatGPT_01_69c5e3f6/`
    - `?? 旧資料_参考_ChatGPT_02_Mirrorea_2025/`
- `rg -n "host_plan_coverage_failure|host_plan did not cover all oracle calls|host_plan_coverage_failures|BundleRunReport|RunReport|BatchRunSummary" ...`
  - `RunReport` / `BundleRunReport` / `BatchRunSummary` と docs / plan / reports における `host_plan_coverage_failure` の出現箇所を確認した。
- `git commit --no-gpg-sign -m "host plan coverage failure の配置境界を整理する"`
  - `[main 3934303] host plan coverage failure の配置境界を整理する`
  - `8 files changed, 229 insertions(+), 6 deletions(-)`
  - `create mode 100644 specs/examples/19-current-l2-host-plan-coverage-failure-placement.md`
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 98 numbered report(s).`
- `git diff --check HEAD`
  - 無出力
- `cargo test -p mir-semantics`
  - unit tests: `2 passed; 0 failed`
  - integration tests: `33 passed; 0 failed`
  - doc-tests: `0 passed; 0 failed`

## 6. Evidence / findings

### 6.1 current code での placement

- `RunReport` は `static_verdict`、`entered_evaluation`、`terminal_outcome`、`trace_audit_sink`、`steps_executed` を持つが、`host_plan_coverage_failure` は持たない。
- `BundleRunReport` は `report: RunReport` だけを持つ thin wrapper であり、`host_plan_coverage_failure` を持たない。
- `host_plan_coverage_failure` は `batch_summary_from_discovery()` が bundle 実行 failure の error text から分類し、
  - `BatchBundleOutcome::Failed { host_plan_coverage_failure: bool }`
  - `BatchRunSummary.host_plan_coverage_failures`
  として materialize する。
- tests でも `run_directory_reports_host_plan_coverage_failures_in_summary()` は summary / batch report でのみこれを確認している。

### 6.2 comparison judgment

- current detached artifact の最小形としては aggregate-only 維持が妥当である。
- ただし将来 typed carrier に昇格させるなら、aggregate-only のままでは coarse すぎる。
- detached non-core / `bundle_context` に薄く置く案は、
  - success artifact にも field を持たせる圧を生む
  - identity / sidecar binding と failure classification を混ぜる
  - current code の実体とずれる
  ため採らない。
- 最も自然な future placement は **bundle failure artifact 側**である。
  - current code が already per-bundle failure bit を持つ
  - payload core を汚さない
  - batch aggregate export はその typed failure を集約する後段に留められる

### 6.3 `fixture authoring / elaboration`

- 依然 independent bottleneck である。
- この task で位置づけは変わらない。
- ただし typed bundle failure artifact があれば、authoring の失敗比較を repo 外で見返しやすくなる。

### 6.4 review fallback

- reviewer tool がこのセッションでは呼べなかった。
- そのため fallback として、source diff inspection と verification command を evidence として残す。
- local review で特に確認したのは次である。
  - `host_plan_coverage_failure` を payload core に上げていないこと
  - `bundle_context` / detached non-core に薄く混ぜていないこと
  - `future typed placement = bundle failure artifact` という判断が current code の `BatchBundleOutcome::Failed` と整合していること
  - `plan/90-source-traceability.md` に新 spec / report を反映していること

## 7. Changes in understanding

変更点は次の 1 点である。

- current detached artifact の settled judgmentとしては `host_plan_coverage_failure` を aggregate-only に残すままでよい。
- その一方で、**将来 typed carrier に昇格させる最小 layer は bundle failure artifact 側**だと source-backed に言える段階まで整理できた。

つまり、current と future-narrow-step を分けて書けるようになった。

## 8. Open questions

- bundle failure artifact の actual schema
- aggregate export がその typed carrier をいつ吸うか
- uncovered call detection をどの段階で string detection から typed reason に置き換えるか
- richer host interface の typed carrier 化との接続点
- detached artifact 保存先と path policy
- reviewer tool が利用できるセッションで、この narrow judgment を reviewer に通すこと

## 9. Suggested next prompt

`specs/examples/19-current-l2-host-plan-coverage-failure-placement.md` を前提に、current L2 parser-free PoC で `host_plan_coverage_failure` を将来 bundle failure artifact 側の typed carrier に昇格させるなら、その最小 schema を docs-only で切り出してください。`RunReport` payload core と bundle-first success artifact は変えず、failure artifact だけを narrow scope で扱ってください。
