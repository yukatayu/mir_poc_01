# Report 0076 — current l2 profile catalog internal integration test boundary

- Date: 2026-04-02T09:58:36.236258Z
- Author / agent: Codex
- Scope: current L2 named profile catalog に関する internal tests / integration tests の責務分担を narrow scope で点検し、public behavior coverage を弱めずに boundary を揃える
- Decision levels touched: L2 companion / verification helper boundary。意味論本体は変更しない

## 1. Objective

`crates/mir-semantics/src/harness.rs` の internal preset table tests と `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs` の named profile integration tests の責務分担を整理する。hard-coded named profile catalog は維持したまま、

- internal tests は single source of truth の整合だけを見る
- integration tests は public behavior を literal expectation で見る
- alias drift と test oracle contamination の余地を減らす

ことを目的とした。

## 2. Inputs consulted

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
- `specs/examples/11-current-l2-selection-helper.md`
- `specs/examples/12-current-l2-selection-profiles.md`
- `specs/examples/13-current-l2-profile-catalog.md`
- `specs/examples/14-current-l2-profile-catalog-externalization.md`
- `docs/reports/0069-current-l2-profile-catalog-single-source-of-truth.md`
- `docs/reports/0070-current-l2-profile-alias-mirror-boundary.md`
- `docs/reports/0071-current-l2-profile-catalog-docs-tests-code-boundary.md`
- `docs/reports/0072-review-helper-boundary-profile-catalog.md`
- `docs/reports/0073-current-l2-named-profile-test-helper-boundary.md`
- `crates/mir-semantics/src/harness.rs`
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`

## 3. Actions taken

1. `code_mapper` で current named profile catalog の internal / integration test 責務を棚卸しした。
2. `harness.rs` の internal tests は private preset table の wiring だけを見ることをコメントで固定した。
3. named profile integration test では、literal な `resolved_request` と unknown alias failure を維持しつつ、selection-shape の比較は `run_directory_profiled` への thin delegation 一致へ寄せた。
4. その変更に対する reviewer で coverage gap が見つかったため、`run_directory_profiled` 側に `runtime-e3` と broad static-only summary の independent coverage を追加した。
5. `specs/examples/13-current-l2-profile-catalog.md` を最小更新し、selected bundle counts / concrete fixture suffix は主に profile helper 側 integration tests が持ち、named profile integration tests は literal request に基づく profiled execution への委譲一致を見ると明記した。
6. narrow reviewer を 2 回実施した。最初の review は finding 1 件、fix 後の follow-up review は `no findings` だった。

## 4. Files changed

- `crates/mir-semantics/src/harness.rs`
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
- `specs/examples/13-current-l2-profile-catalog.md`
- `docs/reports/0074-current-l2-profile-catalog-test-responsibility-boundary.md`
- `docs/reports/0075-current-l2-profile-catalog-followup-review.md`
- `docs/reports/0076-current-l2-profile-catalog-internal-integration-test-boundary.md`

開始時点の既存 dirty state:

- なし

仕様本文コミット:

- `77098e8` `named profile test の責務境界を整理する`

report 自身の commit hash は self-reference の都合で本文に固定していない。

## 5. Commands run and exact outputs

- `git status --short --branch`
  - `## main...origin/main [ahead 3]`
- `cargo test -p mir-semantics run_directory_named_profiles_match_catalog_resolution_and_expected_selection -- --nocapture`
  - RED:
    - `error[E0425]: cannot find function 'assert_named_profile_execution_matches_profiled_summary' in this scope`
  - GREEN:
    - `test run_directory_named_profiles_match_catalog_resolution_and_expected_selection ... ok`
- `cargo test -p mir-semantics`
  - `test harness::tests::named_profile_catalog_aliases_are_derived_from_internal_specs ... ok`
  - `test harness::tests::named_profile_catalog_resolve_is_derived_from_internal_specs ... ok`
  - `running 32 tests`
  - `test result: ok. 32 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s`
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 76 numbered report(s).`
- `git diff --check`
  - 無出力
- reviewer 1
  - finding 1 件。`runtime-e3` と broad static-only coverage が `run_directory_profiled` 側に不足
- reviewer 2
  - `No findings.`

## 6. Evidence / findings

- internal tests の責務は次に絞れた。
  - private `named_profile_specs()` から `ProfileCatalog::aliases()` が導かれること
  - private `named_profile_specs()` から `ProfileCatalog::resolve()` が導かれること
- integration tests の責務は次に絞れた。
  - public alias list の exact compare
  - alias ごとの literal `resolved_request`
  - unknown alias failure
  - literal request から組んだ `SelectionProfile` に対する `run_directory_profiled` との thin delegation 一致
- 1 回目の review で、named profile test から外した selected count / suffix の一部が profile-layer tests にまだ移っていないことが分かった。
- fix として次を追加した。
  - `run_directory_profiled_runtime_e3_runs_one_runtime_bundle`
  - `run_directory_profiled_static_only_includes_profile_name_in_summary`
- これにより、named profile integration test は catalog 固有の public behavior に集中しつつ、selection-shape の具体比較は profile helper 側 integration tests に残せるようになった。
- `ProfileCatalog::resolve()` を integration test oracle に再利用しない current policy も維持できている。

## 7. Changes in understanding

- internal tests と integration tests の重複は「同じ値を見ていること」ではなく、「どの oracle で見ているか」で切るのが自然だった。
- `resolved_request` と unknown alias failure は named profile catalog の public contract そのものなので、integration 側に literal expectation を残す必要がある。
- selected bundle counts と concrete fixture suffix は public machine-check ではあるが、named profile layer の固有責務ではない。current L2 では `run_directory_profiled` 系 integration tests が主に持ち、named profile integration tests は thin delegation の確認に留める方が boundary が明確だった。

## 8. Open questions

- machine-readable catalog asset を再検討する条件は未決定。
- bundle manifest を導入するかは未決定。
- selector grammar / alias grammar の長期固定は未決定。
- path canonicalization policy は未決定。
- detached trace / audit serialization は未決定。
- richer host interface、multi-request scheduler、`Approximate` / `Compensate` も未決定。

## 9. Suggested next prompt

current L2 named profile catalog を前提に、`crates/mir-semantics/src/harness.rs` の internal preset table comment と `specs/examples/13-current-l2-profile-catalog.md` の責務記述が、selection/profile/batch/bundle の他レイヤーでも同じ粒度で揃っているかを横断点検してください。
