# Report 0071 — current l2 profile catalog docs tests code boundary

- Date: 2026-04-02T06:28:04.092397Z
- Author / agent: Codex (GPT-5)
- Scope: current L2 named profile catalog における `specs/examples/13-current-l2-profile-catalog.md` の preset table と `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs` の named profile behavior table の責務分担を整理し、tests 側の helper 化をどこまで許すかを narrow scope で明確化する。
- Decision levels touched: L2 の companion / verification helper 文書。意味論本体は変更しない。

## 1. Objective

hard-coded named profile catalog は維持したまま、docs / tests / code に散っている alias mirror の責務分担を明確化し、drift をさらに減らす。特に次を目的とした。

- docs には何を prose として残すべきかを切る
- tests には何を literal expectation として残すべきかを切る
- code 側 single source of truth から何を派生させてよいかを明記する
- helper 化によって `resolved_request` の public behavior coverage を弱めない
- machine-readable catalog asset や manifest には進まない

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
- `specs/examples/11-current-l2-selection-helper.md`
- `specs/examples/12-current-l2-selection-profiles.md`
- `specs/examples/13-current-l2-profile-catalog.md`
- `specs/examples/14-current-l2-profile-catalog-externalization.md`
- `docs/reports/0065-review-0064-short-rereview.md`
- `docs/reports/0066-current-l2-profile-catalog.md`
- `docs/reports/0067-current-l2-profile-catalog-externalization-comparison.md`
- `docs/reports/0068-review-current-l2-profile-catalog-externalization-narrow-scope.md`
- `docs/reports/0069-current-l2-profile-catalog-single-source-of-truth.md`
- `docs/reports/0070-current-l2-profile-alias-mirror-boundary.md`
- `crates/mir-semantics/src/harness.rs`
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`

## 3. Actions taken

1. `code_mapper` で alias mirror の棚卸しを行い、code / docs / tests の重複箇所と dirty state なしを確認した。
2. `specs/examples/13-current-l2-profile-catalog.md` を current L2 の prose 正本として維持しつつ、docs / tests / code の責務分担を明文化した。
3. 同文書の `representative fixture` 節では、具体的な selected bundle counts や concrete fixture suffix を prose から外し、machine-check の責務として tests 側へ寄せた。
4. `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs` では、`NamedProfileBehaviorCase` の `expected_request` が catalog 実装から独立した literal expectation であることをコメントで固定した。
5. narrow scope reviewer を 1 回実施し、`no findings` を確認した。

## 4. Files changed

- `specs/examples/13-current-l2-profile-catalog.md`
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
- `docs/reports/0071-current-l2-profile-catalog-docs-tests-code-boundary.md`

## 5. Commands run and exact outputs

- `python3 scripts/new_report.py --slug current-l2-profile-catalog-docs-tests-code-boundary`
  - Output: `/home/yukatayu/dev/mir_poc_01/docs/reports/0071-current-l2-profile-catalog-docs-tests-code-boundary.md`
- `cargo check -p mir-semantics`
  - Output: `Finished \`dev\` profile [unoptimized + debuginfo] target(s) in 0.34s`
- `cargo test -p mir-semantics`
  - Output included:
    - `test harness::tests::named_profile_catalog_aliases_are_derived_from_internal_specs ... ok`
    - `test harness::tests::named_profile_catalog_resolve_is_derived_from_internal_specs ... ok`
    - `test named_profile_catalog_lists_expected_aliases ... ok`
    - `test run_directory_named_profiles_match_catalog_resolution_and_expected_selection ... ok`
    - `test run_directory_named_profile_rejects_unknown_alias ... ok`
    - `test result: ok. 30 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s`
- `python3 scripts/validate_docs.py`
  - Output: `Documentation scaffold looks complete.` / `Found 71 numbered report(s).`
- `git diff --check`
  - Output: なし
- `git commit --no-gpg-sign -m "named profile の docs と tests の責務境界を整理する"`
  - Output: `[main e63abdd] named profile の docs と tests の責務境界を整理する`

## 6. Evidence / findings

- 作業開始時点の dirty state はなし。`code_mapper` の確認では `git status --short --branch` は `## main...origin/main` のみだった。
- code 側の single source of truth は引き続き `crates/mir-semantics/src/harness.rs` の hard-coded preset table にある。
- docs 側で正本として残す範囲は次に絞った。
  - alias 一覧
  - alias→`SelectionRequest` の prose table
  - `resolved_request` を含む summary shape
  - helper boundary の説明
- tests 側で literal expectation として残す範囲は次に絞った。
  - alias 一覧の exact compare
  - alias ごとの `resolved_request`
  - selected bundle counts
  - single-fixture alias の concrete fixture suffix
  - unknown alias failure
- helper 化してよいのは、`SelectionRequest` を組み立てる builder 関数や case table までである。`ProfileCatalog::resolve()` を test oracle に再利用しない境界をコメントでも固定した。
- narrow reviewer は `no findings` だった。
- 仕様本文コミットは次の 1 本である。
  - `e63abdd` `named profile の docs と tests の責務境界を整理する`
- この report 自身の commit hash は self-reference の都合で本文に固定していない。

## 7. Changes in understanding

- current L2 の alias mirror は、docs と tests の両方に残るが、両者の責務は同じではない。
- `specs/examples/13-current-l2-profile-catalog.md` は alias の意味と alias→request prose を持つ正本として十分であり、concrete fixture counts まで抱える必要はなかった。
- tests 側は behavior table を持っていてよいが、その table は catalog 実装から派生させず、public behavior を literal に固定する役割に留めるのが最小だった。

## 8. Open questions

- machine-readable catalog asset を再検討する条件は、引き続き `specs/examples/14-current-l2-profile-catalog-externalization.md` に従って **未決定** のまま残す。
- bundle manifest を導入するか、selector grammar / alias grammar を長期固定するか、path canonicalization policy、detached trace / audit serialization、richer host interface、multi-request scheduler、`Approximate` / `Compensate` は今回も **未決定** のままである。

## 9. Suggested next prompt

current L2 named profile catalog を前提に、`crates/mir-semantics/tests/current_l2_minimal_interpreter.rs` の named profile behavior table から selected bundle counts と concrete fixture suffix をどこまで共通 assertion helper に寄せてよいかを比較してください。`resolved_request` の literal expectation と unknown alias failure coverage は維持したまま、test noise だけをさらに減らす範囲に絞ると進めやすいです。
