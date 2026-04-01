# Report 0064 — current L2 selection profiles

- Date: 2026-04-01
- Author / agent: Codex
- Scope: current L2 selection helper の上に載る profile helper の追加
- Decision levels touched: L2

## 1. Objective

bundle manifest を導入せずに、`runtime-only` / `static-only` / `single-fixture` の primitive selection を組み合わせた profile request を 1 つの helper で受け、profile 名付き summary とともに selected batch 実行できるようにする。

## 2. Scope and assumptions

- Mir-0 / Mir-1 / Mirrorea の境界は変更しない。
- profile helper は selection helper の上に薄く載る verification helper に留め、production runner にはしない。
- sidecar discovery と runtime/static-only classification は既存の bundle loader / batch runner / selection helper に残す。
- `single-fixture` の unknown selector は hidden skip にせず error のまま維持する。
- `must_explain` は machine-check に上げず、human-facing explanation obligation に残す。
- task 開始時点の既存 dirty state はなく、`git status --short --branch` は `## main...origin/main` のみだった。

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
- `specs/examples/08-current-l2-host-plan-schema.md`
- `specs/examples/09-current-l2-bundle-loader.md`
- `specs/examples/10-current-l2-batch-runner.md`
- `specs/examples/11-current-l2-selection-helper.md`
- `docs/reports/0058-review-0056-short-rereview.md`
- `docs/reports/0059-current-l2-host-plan-sidecar-loader.md`
- `docs/reports/0060-current-l2-bundle-loader.md`
- `docs/reports/0061-review-0060-short-rereview.md`
- `docs/reports/0062-current-l2-batch-runner.md`
- `docs/reports/0063-current-l2-selection-helper.md`
- `crates/mir-semantics/src/lib.rs`
- `crates/mir-semantics/src/harness.rs`
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`

## 4. Actions taken

1. `code_mapper` を使って、dirty state が無いことと、profile layer の自然な差し込み位置が `crates/mir-semantics/src/harness.rs` であることを確認した。
2. existing selection helper の primitive mode を増殖させず、`SelectionRequest` / `SelectionProfile` / `ProfileRunSummary` を追加する方針に絞った。
3. `SelectionScope` を `runtime-only` / `static-only` の bundle class filter として導入し、`single-fixture` は既存 `SingleFixtureSelector` を再利用する形にした。
4. `select_bundles_from_request` を追加し、scope filter と single-fixture filter を既存 `select_bundles_from_discovery` の逐次合成として実装した。
5. `run_directory_profiled` を追加し、`profile_name` を付けた selected batch 実行 summary を返すようにした。
6. 既存 `run_directory_selected` は `SelectionRequest::from_mode(...)` 経由に寄せ、primitive selection と profile selection の実装を二重化しないようにした。
7. tests を追加し、次を確認できるようにした。
   - `runtime-only + single-fixture`
   - `static-only + single-fixture`
   - `runtime-only + path-selector`
   - `profile_name` の summary 反映
   - unknown selector error
8. `specs/examples/12-current-l2-selection-profiles.md` を追加し、profile helper の最小責務と selection helper との役割分担を整理した。
9. `specs/examples/11-current-l2-selection-helper.md`、`Documentation.md`、`specs/00-document-map.md`、`specs/10-open-questions.md`、`specs/12-decision-register.md` に mirror を反映した。
10. narrow scope reviewer を依頼したが completion は回収できず、local boundary check を併記する方針にした。

## 5. Evidence / outputs / test results

### Files changed

- 仕様本文 / 実装本文 commit `f918a62` に含めた変更:
  - 更新: `Documentation.md`
  - 更新: `crates/mir-semantics/src/harness.rs`
  - 更新: `crates/mir-semantics/src/lib.rs`
  - 更新: `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
  - 更新: `specs/00-document-map.md`
  - 更新: `specs/10-open-questions.md`
  - 更新: `specs/12-decision-register.md`
  - 更新: `specs/examples/11-current-l2-selection-helper.md`
  - 新規: `specs/examples/12-current-l2-selection-profiles.md`
- report commit に含める変更:
  - 新規: `docs/reports/0064-current-l2-selection-profiles.md`

### Commands run and exact outputs

task 着手時:

```bash
git status --short --branch
```

```text
## main...origin/main
```

verification:

```bash
cargo check -p mir-semantics
```

```text
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.32s
```

```bash
cargo test -p mir-semantics
```

```text
running 27 tests
...
test result: ok. 27 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s
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

code commit:

```bash
git -c commit.gpgsign=false commit -m "current L2 の selection profile helper を追加する"
```

```text
[main f918a62] current L2 の selection profile helper を追加する
```

review attempt:

```text
narrow reviewer を依頼したが completion を回収できなかった。local inspection の範囲では追加 finding は無かった。
```

### Findings

- profile layer の最小責務は、primitive selection の合成、profile 名付き request、selected batch 実行、profile 名付き summary の 4 点で足りた。
- sidecar discovery と runtime/static-only classification は `discover_bundles_in_directory` / `select_bundles_from_discovery` / `run_directory` 系へ残っており、profile helper 側で再実装していない。
- `single-fixture` の unknown selector は `selected fixture was not found` error を維持しており、hidden skip にはしていない。
- pre-classification discovery failure を hidden に落とさない current L2 方針は、existing selection helper の逐次合成により profile helper でも維持される。
- machine-check は static/runtime/trace-audit/host-plan coverage に留め、`must_explain` は profile helper でも machine-check に上げていない。

## 6. What changed in understanding

- selection helper の上に profile layer を足す場合でも、primitive mode を新しく増やす必要はなく、request carrier と summary wrapper だけで十分に実験ループを細かくできることが分かった。
- selection と profile を分離しておくと、sidecar discovery や classification を profile helper が抱え込まずに済み、verification helper の層構造が崩れにくい。

## 7. Open questions

- bundle manifest を導入するかは **未決定**。
- selector grammar を長期固定するかは **未決定**。
- path canonicalization policy は **未決定**。
- detached trace / audit serialization は **未決定**。
- richer host interface、multi-request scheduler、`Approximate` / `Compensate` は **未決定**。
- reviewer completion が返らない場合の evidence policy をどこまで標準化するかは **未決定**。

## 8. Suggested next prompt

current L2 selection profile helper を前提に、small named profile catalog か lightweight preset table を追加し、`smoke-runtime` や `smoke-static` のような human-friendly profile alias で batch 実行できる helper を検討してください。manifest には進まず、selection request の再利用だけに絞ると current L2 の verification helper 境界を保ちやすいです。
