# Report 0060 — current l2 bundle loader

- Date: 2026-04-01T05:30:15.310054Z
- Author / agent: Codex
- Scope: current L2 parser-free minimal interpreter / host plan sidecar loader を前提にした fixture bundle loader / bundle-level test helper の追加
- Decision levels touched: L2

## 1. Objective

AST fixture JSON、`.host-plan.json` sidecar、`expected_static` / `expected_runtime` / `expected_trace_audit` を 1 組の bundle として扱える loader / helper を追加し、PoC の実験ループを「fixture + sidecar の追加」で回しやすくする。

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
- `docs/reports/0052-review-0051-short-rereview.md`
- `docs/reports/0053-final-review-current-l2-interpreter-skeleton.md`
- `docs/reports/0054-current-l2-parser-free-interpreter-skeleton.md`
- `docs/reports/0055-review-0054-short-rereview.md`
- `docs/reports/0056-current-l2-host-stub-harness.md`
- `docs/reports/0057-final-review-current-l2-host-harness.md`
- `docs/reports/0058-review-0056-short-rereview.md`
- `docs/reports/0059-current-l2-host-plan-sidecar-loader.md`
- `crates/mir-ast/tests/fixtures/current-l2/`
- `crates/mir-semantics/src/lib.rs`
- `crates/mir-semantics/src/harness.rs`
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`

## 3. Actions taken

1. `code_mapper` で current L2 fixture / sidecar / harness / test の接点を確認し、開始時点で既存 dirty state が無いことを確認した。
2. `harness.rs` を bundle の自然な差し込み位置と判断し、`FixtureBundle`、`FixtureRuntimeRequirement`、`BundleRunReport`、`load_bundle_from_fixture_path`、`run_bundle` の最小 API を設計した。
3. 先に failing test を追加し、未実装の bundle API に対して RED を確認した。
4. bundle loader を実装し、runtime fixture は adjacent `.host-plan.json` sidecar を必須、static-only fixture は sidecar 無しでも load 可、という discovery rule を追加した。
5. `run_bundle` で次を一括実行・照合するようにした。
   - static gate
   - runtime execution
   - `event_kinds`
   - formal な `non_admissible_metadata`
   - short `narrative_explanations`
   - host plan coverage
6. `must_explain` は従来どおり machine-check に上げず、bundle helper の比較対象から外した。
7. 既存テストを runtime path では bundle 経由へ寄せ、bundle loader / discovery failure / static-only bundle を確認する test を追加した。
8. 新規補助文書 `specs/examples/09-current-l2-bundle-loader.md` を追加し、bundle の最小 shape、sidecar discovery、machine-check と human-facing explanation の境界を整理した。
9. `Documentation.md`、`specs/00-document-map.md`、`specs/10-open-questions.md`、`specs/12-decision-register.md` に bundle loader の最小 mirror を反映した。
10. final reviewer を依頼した。
11. reviewer completion を 2 回待機したが返らなかった。
12. fresh reviewer を 1 度だけ取り直したが、こちらも completion が返らなかったため close し、未返却として記録した。

## 4. Files changed

- 既存 dirty state: なし。task 開始時点の `git status --short` は空で、今回の差分だけが worktree にある。
- 仕様本文 / 実装本文の変更対象:
  - 更新: `Documentation.md`
  - 更新: `crates/mir-semantics/src/harness.rs`
  - 更新: `crates/mir-semantics/src/lib.rs`
  - 更新: `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
  - 更新: `specs/00-document-map.md`
  - 更新: `specs/10-open-questions.md`
  - 更新: `specs/12-decision-register.md`
  - 新規: `specs/examples/09-current-l2-bundle-loader.md`
  - 上記を記録した仕様本文 / 実装本文 commit: `75eef7f` (`current L2 の fixture bundle loader を追加する`)
- report commit に含める変更:
  - 新規: `docs/reports/0060-current-l2-bundle-loader.md`

## 5. Commands run and exact outputs

task 着手前の `code_mapper` 確認:

```bash
git status --short
```

```text

```

RED:

```bash
cargo test -p mir-semantics
```

```text
error[E0432]: unresolved imports `mir_semantics::FixtureRuntimeRequirement`, `mir_semantics::load_bundle_from_fixture_path`, `mir_semantics::run_bundle`
```

fresh verification:

```bash
cargo check -p mir-semantics
```

```text
Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.45s
```

```bash
cargo test -p mir-semantics
```

```text
running 12 tests
...
test result: ok. 12 passed; 0 failed
```

```bash
python3 scripts/validate_docs.py
```

```text
Documentation scaffold looks complete.
Found 60 numbered report(s).
```

```bash
git diff --check
```

```text

```

```bash
git status --short --branch
```

```text
## main...origin/main [ahead 5]
 M Documentation.md
 M crates/mir-semantics/src/harness.rs
 M crates/mir-semantics/src/lib.rs
 M crates/mir-semantics/tests/current_l2_minimal_interpreter.rs
 M specs/00-document-map.md
 M specs/10-open-questions.md
 M specs/12-decision-register.md
?? docs/reports/0060-current-l2-bundle-loader.md
?? specs/examples/09-current-l2-bundle-loader.md
```

final reviewer:

```text
1回目の wait: timed_out: true, status: {}
2回目の wait: timed_out: true, status: {}
first close: previous_status: running
fresh reviewer wait: timed_out: true, status: {}
second close: previous_status: running
```

## 6. Evidence / findings

- bundle の最小 shape は、`fixture_path`、`fixture`、optional `host_plan_path`、optional `host_plan`、`runtime_requirement` で足りた。
- runtime fixture は adjacent sidecar 必須、static-only fixture は sidecar 無しでも bundle 成立、という discovery rule で E1 / E2 / E3 variant / E6 と E4 / E5 を同じ入口に揃えられた。
- `run_bundle` は host plan coverage を含む `FixtureHostStub::run_fixture` の上に machine-check compare を薄く載せるだけでよく、新しい runtime semantics を増やさずに bundle-level helper を成立させられた。
- `must_explain` を bundle helper の exact compare に入れないことで、current L2 の event / metadata / human-facing explanation の三層分離を維持できた。
- `specs/10-open-questions.md` と `specs/12-decision-register.md` には、bundle manifest や batch discovery を固定しない current L2 の最小 mirror だけを追加した。意味論の本体は変更していない。
- 仕様本文 / 実装本文の commit hash は `75eef7f` である。report 自身の commit hash は self-reference の都合で本文に固定しない。
- final reviewer の結果:
  - completion は返らなかった。
  - current task では local verification を主証跡とし、review attempt 自体は evidence として残す。

## 7. Changes in understanding

- host plan sidecar loader の次の段階として bundle を入れると、runtime fixture と static-only fixture の分岐が tests 側の ad hoc helper から消え、current L2 の実験ループがかなり見通しよくなる。
- bundle helper は新しい semantics layer ではなく、既存の static gate / runtime / expectation compare を束ねる verification edge として切るのが最小だった。
- machine-check 対象を bundle 層で増やしすぎない方が、`must_explain` を narrative obligation に残す current L2 policy と自然に整合する。

## 8. Open questions

- bundle manifest を追加するかどうかは **未決定**。
- fixture directory 全体を batch 実行する API を current L2 に入れるかは **未決定**。
- detached trace / audit serialization は **未決定**。
- richer host interface、multi-request scheduler、`Approximate` / `Compensate` は **未決定**。
- parser 実装前に bundle 層でさらに詰めるべき点として、directory discovery の長期固定と sidecar を持つ static-only fixture の扱いは **未決定**。

## 9. Suggested next prompt

`crates/mir-semantics` の current L2 bundle loader / bundle-level helper を前提に、fixture directory を束ねて一括実行する batch runner を追加してください。parser には進まず、bundle discovery・runtime bundle / static-only bundle の振り分け・summary report 出力だけに絞ると、PoC 実験ループをさらに回しやすくできます。
