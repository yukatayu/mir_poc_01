# Report 0102 — review host plan coverage failure aggregate connection

- Date: 2026-04-03T06:09:12.218706Z
- Author / agent: Codex reviewer pass
- Scope: uncommitted docs-only change for `specs/examples/21-current-l2-host-plan-coverage-failure-aggregate-connection.md` and listed mirrors, reviewed against current code anchors and traceability rules
- Decision levels touched: L2

## 1. Objective

new spec `specs/examples/21-current-l2-host-plan-coverage-failure-aggregate-connection.md` と mirror 更新が、current code anchor、helper boundary、traceability policy と矛盾していないかを narrow scope で確認する。

### Scope and assumptions

- review 対象は uncommitted docs-only change に限定する。
- 特に次の 4 点だけを見る。
  - current code anchor (`crates/mir-semantics/src/lib.rs`, `crates/mir-semantics/src/harness.rs`, tests) との整合
  - 未決 implementation detail を settled fact として書いていないか
  - aggregate を coarse に保ち、bundle failure artifact を per-bundle owner に保てているか
  - document-map / plan traceability の mirror gap
- code や既存 docs は変更しない前提で review した。

## 2. Inputs consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/20-current-l2-host-plan-coverage-failure-bundle-failure-artifact-schema.md`
- `specs/examples/21-current-l2-host-plan-coverage-failure-aggregate-connection.md`
- `plan/00-index.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `docs/reports/0100-host-plan-coverage-failure-bundle-failure-artifact-schema.md`
- `docs/reports/0101-review-host-plan-coverage-failure-bundle-failure-artifact-schema.md`
- `crates/mir-semantics/src/lib.rs`
- `crates/mir-semantics/src/harness.rs`
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`

## 3. Actions taken

1. AGENTS の読書順に従って repository docs と relevant `plan/` を再読した。
2. scoped files の uncommitted diff を確認した。
3. `host_plan_coverage_failure`、`BatchBundleOutcome::Failed`、`BatchRunSummary`、`batch_summary_from_discovery()` の code anchor を確認した。
4. tests がどの layer で error text / failure classification を固定しているかを再確認した。
5. traceability と report chain の mirror を確認した。
6. `plan/ 更新不要`。この review task 自体では repository memory を更新しない。

## 4. Files changed

- `docs/reports/0102-review-host-plan-coverage-failure-aggregate-connection.md`

## 5. Commands run and exact outputs

- `git status --short`
  - scoped files は user 指定どおり uncommitted state だった。
  - `specs/examples/21-current-l2-host-plan-coverage-failure-aggregate-connection.md` は untracked。
- `git diff --stat -- Documentation.md specs/00-document-map.md specs/examples/20-current-l2-host-plan-coverage-failure-bundle-failure-artifact-schema.md specs/examples/21-current-l2-host-plan-coverage-failure-aggregate-connection.md plan/07-parser-free-poc-stack.md plan/09-helper-stack-and-responsibility-map.md plan/11-roadmap-near-term.md plan/12-open-problems-and-risks.md plan/90-source-traceability.md`
  - 8 files changed, 27 insertions, 6 deletions。
- `rg -n "host_plan_coverage_failure|batch_summary_from_discovery|BatchRunSummary|BatchBundleOutcome::Failed|BundleExecutionFailure" crates/mir-semantics/src/harness.rs crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
  - `host_plan_coverage_failure` の current materialization point と relevant tests を抽出した。
- `ls docs/reports | tail -n 20`
  - `docs/reports/` 末尾は `0101-review-host-plan-coverage-failure-bundle-failure-artifact-schema.md` までで、`0102` / `0103` は存在しなかった時点で review を開始した。

## 6. Evidence / findings

### Finding 1

`specs/examples/21-current-l2-host-plan-coverage-failure-aggregate-connection.md` の current code anchor 説明で、「tests でも summary / batch report 側だけを literal expectation にしている」とあるが、実際の test suite は harness layer でも uncovered oracle call の raw message を直接固定している。`batch_summary_from_discovery()` が `error_text.contains("host plan did not cover all oracle calls")` で分類している以上、この違いは migration scope の見積りを誤らせる。summary / batch report だけの問題ではなく、harness error wording も既存 anchor である。
Evidence:
- `specs/examples/21-current-l2-host-plan-coverage-failure-aggregate-connection.md:27-28`
- `crates/mir-semantics/src/harness.rs:813-815`
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs:427-435`
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs:548-555`

### Finding 2

`specs/00-document-map.md` の新規 entry は、spec 21 を aggregate connection comparison と説明した直後に、「failure artifact の docs-only schema refinement だけを与える」と書いており、spec 20 用の説明がそのままぶら下がっている。spec 21 本文は aggregate export との connection judgment を扱っており、failure artifact schema refinement そのものではないため、document map の mirror がずれている。
Evidence:
- `specs/00-document-map.md:91-93`
- `specs/examples/21-current-l2-host-plan-coverage-failure-aggregate-connection.md:5-7`

### Finding 3

`plan/90-source-traceability.md` は parser-free PoC report chain に `0102` と `0103` を追加していたが、review 開始時点では対応する report file が存在しなかった。この review で必須 report として `0102` は補われたが、`0103` は依然として存在しない。repo policy では non-trivial task ごとに新 report が必要なので、traceability が先走っていた点は finding のままである。
Evidence:
- `plan/90-source-traceability.md:84-88`
- `AGENTS.md:22-25`
- `docs/reports/` listing at review start ended at `0101-review-host-plan-coverage-failure-bundle-failure-artifact-schema.md`
- current workspace now has `0102-review-host-plan-coverage-failure-aggregate-connection.md` but no `0103*`

### No finding on helper boundary

aggregate を coarse summary に留め、per-bundle ownership を future bundle failure artifact 側へ寄せる判断自体は、current helper boundary と整合していた。
- `BatchRunSummary` は today でも coarse aggregate と per-bundle report list を持つだけで、typed per-bundle carrier は持たない。
- 新 spec も `fixture_id` / `fixture_path` / `bundle_context` / explanation を aggregate 側へ持ち込まないとしており、helper-boundary drift は見当たらなかった。

## 7. Changes in understanding

- 新 spec の主判断自体よりも、current code anchor と mirror prose の精度が review の中心論点だった。
- typed aggregate を histogram / kind count に留める boundary judgment は、current helper stack と整合している。
- ただし current implementation は string-based classification に依存し、しかもその string は harness-level test でも固定されているため、将来の typed migration は spec 21 の prose より少し強く current wording に縛られている。

## 8. Open questions

- `BatchRunSummary` の future typed aggregate を current list / bool shape と併存させるか、置き換えるか。
- string detection から typed detection へ移るとき、harness-level error wording test をどう扱うか。
- spec 21 に対応する authoring report と review report をどの numbering で正規化するか。

## 9. Suggested next prompt

`specs/examples/21-current-l2-host-plan-coverage-failure-aggregate-connection.md` の review findings を反映してください。特に、current test anchor が harness error text も固定している点を prose に反映し、`specs/00-document-map.md` の spec 21 mirror を aggregate connection comparison に合わせて直し、`plan/90-source-traceability.md` の report chain を実在する report file と一致させてください。
