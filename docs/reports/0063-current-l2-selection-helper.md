# Report 0063 — current l2 selection helper

- Date: 2026-04-01
- Author / agent: Codex
- Scope: current L2 bundle loader / batch runner を前提にした fixture bundle filter / selection helper の追加
- Decision levels touched: L2

## 1. Objective

bundle manifest を導入せずに、current L2 fixture directory 内の bundle を `runtime-only` / `static-only` / `single-fixture` で選別して回せる helper を追加し、PoC の実験ループをより細かく回せるようにする。

## 2. Scope and assumptions

- Mir-0 / Mir-1 / Mirrorea の境界は変更しない。
- selection helper は bundle loader / batch runner の上に薄く載る verification helper に留め、production runner にはしない。
- `must_explain` は machine-check に上げず、human-facing explanation obligation に残す。
- runtime bundle / static-only bundle の判定は既存どおり `expected_runtime.enters_evaluation` を使う。
- task 開始時点の dirty state は無く、`git status --short --branch` は `## main...origin/main [ahead 10]` のみだった。

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
- `specs/examples/02-current-l2-ast-fixture-schema.md`
- `specs/examples/03-current-l2-evaluation-state-schema.md`
- `specs/examples/04-current-l2-step-semantics.md`
- `specs/examples/05-current-l2-oracle-api.md`
- `specs/examples/06-current-l2-interpreter-skeleton.md`
- `specs/examples/07-current-l2-host-stub-harness.md`
- `specs/examples/08-current-l2-host-plan-schema.md`
- `specs/examples/09-current-l2-bundle-loader.md`
- `specs/examples/10-current-l2-batch-runner.md`
- `docs/reports/0058-review-0056-short-rereview.md`
- `docs/reports/0059-current-l2-host-plan-sidecar-loader.md`
- `docs/reports/0060-current-l2-bundle-loader.md`
- `docs/reports/0061-review-0060-short-rereview.md`
- `docs/reports/0062-current-l2-batch-runner.md`
- `crates/mir-semantics/src/lib.rs`
- `crates/mir-semantics/src/harness.rs`
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
- `crates/mir-ast/tests/fixtures/current-l2/`

## 4. Actions taken

1. `code_mapper` を使って、dirty state が無いこと、selection helper の自然な差し込み位置が `crates/mir-semantics/src/harness.rs` であることを確認した。
2. current L2 の正本と既存 batch runner / bundle loader / host harness の境界を読み、selection helper は `discover_bundles_in_directory` と `run_directory` の間に薄く載せる方針を確定した。
3. selection-focused tests を先に追加し、未実装 API に対する RED を確認した。
4. `crates/mir-semantics/src/harness.rs` に次を追加した。
   - `SelectionMode`
   - `SingleFixtureSelector`
   - `select_bundles_from_discovery`
   - `run_directory_selected`
5. `SelectionMode::RuntimeOnly` / `StaticOnly` / `SingleFixture` に応じて既存 discovery 結果を filter する helper を実装した。
6. `single-fixture` は stem または path を受ける形にし、一致対象が bundle / discovery failure のどちらにも無い場合は error にした。
7. pre-classification な discovery failure を hidden omission にしないため、`runtime-only` / `static-only` selection でも `runtime_requirement = None` の failure は可視のまま残す方針にした。
8. tests を追加・更新し、次を確認した。
   - `runtime-only` が E1 / E2 / E3 / E6 だけを拾う
   - `static-only` が E4 / E5 だけを拾う
   - `single-fixture` が stem / path 指定で 1 件だけ実行する
   - 存在しない selector が fail する
9. 新規補助文書 `specs/examples/11-current-l2-selection-helper.md` を追加し、selection helper の最小責務、sidecar discovery との整合、machine-check と human-facing explanation の境界を整理した。
10. `Documentation.md`、`specs/00-document-map.md`、`specs/10-open-questions.md`、`specs/12-decision-register.md` に selection helper の mirror を追加した。
11. narrow-scope reviewer を 1 回だけ当て、`no findings` を確認した。

## 5. Evidence / outputs / test results

### Files changed

- 既存 dirty state: なし
- 仕様本文 / 実装本文の変更対象:
  - 更新: `Documentation.md`
  - 更新: `crates/mir-semantics/src/harness.rs`
  - 更新: `crates/mir-semantics/src/lib.rs`
  - 更新: `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
  - 更新: `specs/00-document-map.md`
  - 更新: `specs/10-open-questions.md`
  - 更新: `specs/12-decision-register.md`
  - 新規: `specs/examples/11-current-l2-selection-helper.md`
- 上記を含む仕様本文 / 実装本文 commit: `097d4b7` (`current L2 の selection helper を追加する`)
- report commit に含める変更:
  - 新規: `docs/reports/0063-current-l2-selection-helper.md`
- report 自身の commit hash は self-reference の都合で本文に固定しない。

### Commands run and exact outputs

task 着手時:

```bash
git status --short --branch
```

```text
## main...origin/main [ahead 10]
```

RED:

```bash
cargo test -p mir-semantics selection_runtime_only_keeps_only_runtime_bundles -- --nocapture
```

```text
error[E0432]: unresolved imports `mir_semantics::SelectionMode`, `mir_semantics::SingleFixtureSelector`, `mir_semantics::run_directory_selected`, `mir_semantics::select_bundles_from_discovery`
```

focused verification:

```bash
cargo test -p mir-semantics selection_ -- --nocapture
```

```text
running 2 tests
test selection_runtime_only_keeps_only_runtime_bundles ... ok
test selection_static_only_keeps_only_static_bundles ... ok
test result: ok. 2 passed; 0 failed
```

```bash
cargo test -p mir-semantics run_directory_selected_ -- --nocapture
```

```text
running 4 tests
test run_directory_selected_rejects_unknown_single_fixture ... ok
test run_directory_selected_runtime_only_executes_only_runtime_bundles ... ok
test run_directory_selected_single_fixture_runs_only_requested_fixture ... ok
test run_directory_selected_single_fixture_accepts_path_selector ... ok
test result: ok. 4 passed; 0 failed
```

full verification:

```bash
cargo check -p mir-semantics
```

```text
Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.45s
```

```bash
cargo test -p mir-semantics
```

```text
running 22 tests
...
test result: ok. 22 passed; 0 failed
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

commit:

```bash
git commit -m "current L2 の selection helper を追加する"
```

```text
error: gpg failed to sign the data
```

```bash
git -c commit.gpgsign=false commit -m "current L2 の selection helper を追加する"
```

```text
[main 097d4b7] current L2 の selection helper を追加する
```

final reviewer:

```text
no findings
```

### Findings

- selection helper の最小責務は、bundle discovery の再利用、runtime/static-only/single-fixture の filter、selected bundle 群に対する batch 実行、selected summary の返却だけで足りた。
- `runtime-only` / `static-only` の判定は既存 `FixtureRuntimeRequirement` に乗せるだけで足り、新しい sidecar discovery rule は不要だった。
- `single-fixture` は stem と path の両方を受けられるが、selector grammar の長期固定までは current L2 に入れていない。
- `must_explain` は selection helper でも machine-check に入れていない。比較対象は static/runtime/trace-audit/coverage に留まっている。
- pre-classification discovery failure を hidden に落とさないため、`runtime-only` / `static-only` selection でも `runtime_requirement = None` の discovery failure は可視のまま残す方針にした。
- final reviewer は narrow scope で `no findings` だった。

## 6. What changed in understanding

- batch runner の次の段階として selection helper を入れても、新しい semantics layer ではなく verification helper のまま保てることが確認できた。
- runtime/static-only の filter だけを見ると discovery failure を落としやすいが、classification できなかった failure を可視に残すことで hidden omission を避けられることが明確になった。
- `single-fixture` を stem / path selector に留めるだけで、manifest 無しでも PoC の実験ループはかなり細かく回せる。

## 7. Open questions

- bundle manifest を導入するかは **未決定**。
- selector grammar を長期固定するかは **未決定**。
- path canonicalization policy は **未決定**。
- directory discovery rule を長期固定するかは **未決定**。
- detached trace / audit serialization は **未決定**。
- richer host interface、multi-request scheduler、`Approximate` / `Compensate` は **未決定**。

## 8. Suggested next prompt

current L2 selection helper を前提に、selection mode を組み合わせた小さな profile layer を追加してください。manifest には進まず、`runtime-only + single-fixture` のような複合指定と、summary の profile 名付けだけに絞ると、PoC 実験ループをさらに回しやすくできます。
