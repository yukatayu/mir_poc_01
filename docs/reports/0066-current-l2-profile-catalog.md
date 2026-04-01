# Report 0066 — current l2 profile catalog

- Date: 2026-04-01T09:19:51.373060Z
- Author / agent: Codex
- Scope: current L2 selection profile helper の上に small named profile catalog / preset table を追加する
- Decision levels touched: L2

## 1. Objective

bundle manifest をまだ導入せずに、既存の `SelectionRequest` / `SelectionProfile` の上へ human-friendly alias を薄く載せる current L2 named profile catalog を追加する。対象 alias は最低限 `smoke-runtime`、`smoke-static`、`runtime-e3`、`static-e4` とし、PoC 実験ループを短い profile 名で回せるようにする。

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
- `specs/examples/09-current-l2-bundle-loader.md`
- `specs/examples/10-current-l2-batch-runner.md`
- `specs/examples/11-current-l2-selection-helper.md`
- `specs/examples/12-current-l2-selection-profiles.md`
- `docs/reports/0058-review-0056-short-rereview.md`
- `docs/reports/0059-current-l2-host-plan-sidecar-loader.md`
- `docs/reports/0060-current-l2-bundle-loader.md`
- `docs/reports/0061-review-0060-short-rereview.md`
- `docs/reports/0062-current-l2-batch-runner.md`
- `docs/reports/0063-current-l2-selection-helper.md`
- `docs/reports/0064-current-l2-selection-profiles.md`
- `crates/mir-semantics/src/harness.rs`
- `crates/mir-semantics/src/lib.rs`
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`

## 3. Actions taken

- selection helper / profile helper の既存 API を読み、catalog をその上に薄く差し込む位置を確認した。
- `ProfileCatalog`、`NamedProfileRunSummary`、`run_directory_named_profile` を `crates/mir-semantics/src/harness.rs` に追加した。
- alias を次の既存 request へ解決する preset table を実装した。
  - `smoke-runtime`
  - `smoke-static`
  - `runtime-e3`
  - `static-e4`
- `resolved_request` を summary に含める wrapper を追加し、selection/profile helper 自体は肥大化させない形にした。
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs` に alias-focused tests を追加した。
- `specs/examples/13-current-l2-profile-catalog.md` を新設し、関連する docs mirror を更新した。
- 0064 の short re-review を別 report `0065` に記録した。

## 4. Files changed

- `Documentation.md`
- `crates/mir-semantics/src/harness.rs`
- `crates/mir-semantics/src/lib.rs`
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
- `specs/00-document-map.md`
- `specs/10-open-questions.md`
- `specs/12-decision-register.md`
- `specs/examples/12-current-l2-selection-profiles.md`
- `specs/examples/13-current-l2-profile-catalog.md`
- `docs/reports/0065-review-0064-short-rereview.md`
- `docs/reports/0066-current-l2-profile-catalog.md`

開始時の既存 dirty state:

- なし

## 5. Commands run and exact outputs

1. `python3 scripts/new_report.py --slug review-0064-short-rereview && python3 scripts/new_report.py --slug current-l2-profile-catalog`
   - `Created docs/reports/0065-review-0064-short-rereview.md`
   - `Created docs/reports/0066-current-l2-profile-catalog.md`
2. `cargo test -p mir-semantics named_profile_catalog_lists_expected_aliases -- --nocapture`
   - 初回は unresolved import で失敗し、RED として catalog API の不足を確認した。
3. `cargo check -p mir-semantics`
   - 成功
4. `cargo test -p mir-semantics`
   - 成功
5. `python3 scripts/validate_docs.py`
   - `Documentation scaffold looks complete.`
   - `Found 66 numbered report(s).`
6. `git diff --check`
   - 無出力

## 6. Evidence / findings

主な evidence:

- catalog layer は `ProfileCatalog::resolve` で alias を既存 `SelectionProfile` / `SelectionRequest` へ解決するだけで、sidecar discovery や runtime/static-only classification を再実装していない。
- `run_directory_named_profile` は既存 `run_directory_profiled` を呼び出し、`resolved_request` を summary に付加する wrapper に留まっている。
- unknown alias は `unknown named profile alias: ...` error として返り、hidden skip には落ちない。
- tests で次を確認した。
  - `smoke-runtime` が `runtime-only` と同じ selected counts を返す。
  - `smoke-static` が `static-only` と同じ selected counts を返す。
  - `runtime-e3` が E3 runtime bundle のみを選ぶ。
  - `static-e4` が E4 static-only bundle のみを選ぶ。
  - 未知 alias が適切に fail する。

判断:

- named profile catalog は current L2 selection profile helper の上に載る薄い alias layer として十分であり、selection/profile/batch/bundle の責務境界を崩していない。
- `must_explain` は引き続き machine-check に上げず、人間向け explanation obligation に残している。

関連 commit:

- `b3acda5` `current L2 の named profile catalog を追加する`
- `93837b1` `named profile catalog の作業報告を追加する`
  - report 自身の commit hash は self-reference の都合で執筆時点には固定できないため、ここでは追記時点の hash として記録する。

## 7. Changes in understanding

- `resolved_request` を summary に含めるだけで、人間向け alias 層を追加しても selection/profile helper 自体を膨らませずに済むことが確認できた。
- alias 層は path canonicalization や manifest を要せず、current L2 PoC の短い実験ループに十分な最小形であることが見えた。

## 8. Open questions

- bundle manifest を導入するかどうかは **未決定**。
- alias grammar / selector grammar を長期固定するかどうかは **未決定**。
- path canonicalization policy は **未決定**。
- detached trace / audit serialization、richer host interface、multi-request scheduler、`Approximate` / `Compensate` は **未決定**。

## 9. Suggested next prompt

current L2 selection profile helper と named profile catalog を前提に、catalog alias を sidecar ではなく小さな preset manifest へ外出しする必要があるかを比較整理してください。現時点では semantics を変えず、profile catalog を hard-coded table に留めるか、machine-readable catalog asset へ進めるかだけを検討対象にしてください。
