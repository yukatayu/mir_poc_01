# Report 0062 — current l2 batch runner

- Date: 2026-04-01T06:42:09.367997Z
- Author / agent: Codex
- Scope: current L2 parser-free minimal interpreter / bundle loader を前提にした fixture directory batch runner / directory discovery / summary helper の追加
- Decision levels touched: L2

## 1. Objective

current L2 fixture directory から bundle を自動 discovery し、runtime bundle / static-only bundle を振り分け、static gate / runtime outcome / trace-audit expectation / host plan coverage を directory 単位で一括実行する最小 batch runner を追加する。

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
- `docs/reports/0055-review-0054-short-rereview.md`
- `docs/reports/0056-current-l2-host-stub-harness.md`
- `docs/reports/0057-final-review-current-l2-host-harness.md`
- `docs/reports/0058-review-0056-short-rereview.md`
- `docs/reports/0059-current-l2-host-plan-sidecar-loader.md`
- `docs/reports/0060-current-l2-bundle-loader.md`
- `docs/reports/0061-review-0060-short-rereview.md`
- `crates/mir-ast/tests/fixtures/current-l2/`
- `crates/mir-semantics/src/lib.rs`
- `crates/mir-semantics/src/harness.rs`
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`

## 3. Actions taken

1. `0060` の短い re-review を先に走らせ、local spot-check では no findings だったことを `0061` に記録した。
2. batch runner の既存差分を確認し、bundle loader の薄い上位層として次を持つ方針を採った。
   - `discover_bundles_in_directory`
   - `run_directory`
   - `BundleDiscoveryReport`
   - `BatchRunSummary`
3. runtime bundle / static-only bundle の判定を `expected_runtime.enters_evaluation` へ寄せ、sidecar discovery は既存の adjacent `.host-plan.json` rule を再利用した。
4. discovery failure と run failure を分け、さらに host plan coverage failure を独立集計できる summary shape にした。
5. `current_l2_minimal_interpreter.rs` に batch 用 RED/GREEN tests を追加し、少なくとも次を確認できるようにした。
   - directory discovery
   - runtime/static-only classification
   - batch summary
   - missing sidecar discovery failure
   - host-plan coverage failure の別集計
6. `specs/examples/10-current-l2-batch-runner.md` を追加し、batch runner の最小責務、sidecar discovery、machine-check と human-facing explanation の境界を整理した。
7. `Documentation.md`、`specs/00-document-map.md`、`specs/10-open-questions.md`、`specs/12-decision-register.md` に current L2 mirror を反映した。
8. verification を通し、final reviewer も試行する。

## 4. Files changed

- 既存 dirty state:
  - task 開始時点ではなし。`git status --short --branch` は `## main...origin/main [ahead 7]` のみだった。
- 仕様本文 / 実装本文の変更対象:
  - 更新: `Documentation.md`
  - 更新: `crates/mir-semantics/src/harness.rs`
  - 更新: `crates/mir-semantics/src/lib.rs`
  - 更新: `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
  - 更新: `specs/00-document-map.md`
  - 更新: `specs/10-open-questions.md`
  - 更新: `specs/12-decision-register.md`
  - 新規: `specs/examples/10-current-l2-batch-runner.md`
  - 上記を含む仕様本文 / 実装本文 commit: `b00ca03` (`current L2 の batch runner を追加する`)
- report / review 記録:
  - 新規: `docs/reports/0061-review-0060-short-rereview.md`
  - 新規: `docs/reports/0062-current-l2-batch-runner.md`

## 5. Commands run and exact outputs

task 着手時:

```bash
git status --short --branch
```

```text
## main...origin/main [ahead 7]
```

RED:

```bash
cargo test -p mir-semantics discover_bundles_in_directory_classifies_runtime_and_static_only_fixtures -- --nocapture
```

```text
error[E0432]: unresolved imports `mir_semantics::discover_bundles_in_directory`, `mir_semantics::run_directory`
```

GREEN:

```bash
cargo test -p mir-semantics -- --nocapture
```

```text
running 16 tests
...
test result: ok. 16 passed; 0 failed
```

```bash
python3 scripts/validate_docs.py
```

```text
Documentation scaffold looks complete.
Found 63 numbered report(s).
```

```bash
git diff --check
```

```text

```

## 6. Evidence / findings

- batch runner の最小責務は、bundle discovery、runtime/static-only classification、bundle execution、summary report の 4 点で足りた。
- runtime bundle は adjacent sidecar 必須、static-only bundle は sidecar 無しでも通す current L2 rule を保ったまま、directory 単位の helper を薄く積み増せた。
- machine-check は引き続き次に限定できた。
  - static verdict
  - runtime final outcome
  - `event_kinds`
  - formal な `non_admissible_metadata`
  - short `narrative_explanations`
- `must_explain` は batch runner でも machine-check に上げていない。current L2 の event / metadata / human-facing explanation 分離は維持されている。
- summary report では少なくとも次が機械的に読める。
  - `total_bundles`
  - `runtime_bundles`
  - `static_only_bundles`
  - `passed`
  - `failed`
  - `discovery_failures`
  - `host_plan_coverage_failures`
- final reviewer は 1 回 spawn したが completion を回収できず、その後の fresh retry は agent thread limit で拒否された。current task では local verification を主証跡として残す。
- report 自身の commit hash は self-reference の都合で本文に固定しない。

## 7. Changes in understanding

- bundle helper の次の段階として batch runner を入れても、新しい semantics layer ではなく verification helper のまま保てることが確認できた。
- discovery failure と run failure を分離しておくと、runtime fixture missing sidecar と host plan coverage miss を summary で自然に区別できる。
- current L2 では manifest を先に入れる必要はなく、directory 直下の fixture discovery だけで PoC の実験ループを十分回せる。

## 8. Open questions

- bundle manifest を導入するかは **未決定**。
- directory discovery rule を長期固定するかは **未決定**。
- detached trace / audit serialization は **未決定**。
- richer host interface、multi-request scheduler、`Approximate` / `Compensate` は **未決定**。
- final reviewer completion が返らない場合の運用記録をどこまで標準化するかは **未決定**。

## 9. Suggested next prompt

current L2 batch runner を前提に、bundle manifest なしでも fixture directory を複数 profile に分けて回せる filter / selection helper を追加してください。parser には進まず、`runtime-only`、`static-only`、`single-fixture` の選別だけを入れると、PoC の実験ループをさらに細かく回せます。
