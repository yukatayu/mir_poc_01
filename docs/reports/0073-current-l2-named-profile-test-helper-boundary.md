# Report 0073 — current L2 named profile test helper boundary

## 1. Title and identifier

- Report 0073
- current L2 named profile test helper boundary

## 2. Objective

`crates/mir-semantics/tests/current_l2_minimal_interpreter.rs` の named profile behavior table について、`resolved_request` と unknown alias failure は literal expectation のまま残しつつ、selected bundle counts と concrete fixture suffix をどこまで tests ローカル helper に寄せてよいかを整理し、drift を減らす。

## 3. Scope and assumptions

- 対象は current L2 named profile catalog 周辺の narrow scope に限る。
- semantics、catalog 実装、selection/profile/batch/bundle の責務境界は変更しない。
- machine-readable catalog asset や manifest は導入しない。
- 開始時点の worktree は clean だった。
- final narrow review は diff/spec conformance review として実施し、runtime test 実行はローカルで別途行った。

## 4. Documents consulted

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
- `docs/reports/0071-current-l2-profile-catalog-docs-tests-code-boundary.md`
- `crates/mir-semantics/src/harness.rs`
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`

## 5. Actions taken

1. `code_mapper` で literal expectation の散在箇所を棚卸しし、alias list / `resolved_request` / selected bundle counts / concrete fixture suffix / unknown alias failure の現状を確認した。
2. tests 側で `ExpectedSelectedBundles` と `assert_named_profile_selected_bundles()` を追加し、selected bundle counts と single-fixture alias の concrete fixture suffix だけを tests ローカル helper に束ねた。
3. `resolved_request` の literal expectation は既存の `expected_*_request()` と case table に残し、catalog 実装由来の oracle へ戻らないよう維持した。
4. unknown alias failure の standalone assertion はそのまま残した。
5. `specs/examples/13-current-l2-profile-catalog.md` を最小更新し、selected bundle counts / concrete fixture suffix は tests ローカル helper に束ねてよいが、`ProfileCatalog::resolve()` を test oracle に再利用してはならないと明記した。
6. ローカルで `cargo test -p mir-semantics`、`python3 scripts/validate_docs.py`、`git diff --check` を実行した。
7. narrow reviewer を実施し、no findings を確認した。review 記録は `docs/reports/0072-review-helper-boundary-profile-catalog.md`。

Files changed:

- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
- `specs/examples/13-current-l2-profile-catalog.md`
- `docs/reports/0072-review-helper-boundary-profile-catalog.md`
- `docs/reports/0073-current-l2-named-profile-test-helper-boundary.md`

仕様本文コミット:

- `b1fc8e2` `named profile behavior table の helper 境界を整理する`

report 自身の commit hash は self-reference の都合で本文に固定していない。

## 6. Evidence / outputs / test results

Commands run and outputs:

- `cargo test -p mir-semantics`
  - `test result: ok. 2 passed; 0 failed; ...`（unit tests）
  - `test result: ok. 30 passed; 0 failed; ...`（integration tests）
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 71 numbered report(s).`
- `git diff --check`
  - 無出力
- narrow reviewer
  - `No findings.`

Evidence summary:

- selected bundle counts と concrete fixture suffix は `ExpectedSelectedBundles` にまとまり、single-fixture alias では `single_runtime(...)` / `single_static(...)` で test noise を減らした。
- `resolved_request` は従来どおり literal builder 関数から exact compare しており、catalog 実装を test oracle に再利用していない。
- unknown alias failure も従来どおり literal error assertion のまま残っている。
- `specs/examples/13-current-l2-profile-catalog.md` は、上記 boundary を prose で追認するだけの最小更新に留めた。

## 7. What changed in understanding

- helper 化してよい範囲は、selected bundle counts と concrete fixture suffix のような machine-check 用 selection-shape expectation までで十分だった。
- `resolved_request` と unknown alias failure は public behavior coverage そのものなので、tests 側に literal expectation を残す必要があるという境界がより明確になった。
- docs の prose は alias 一覧と責務境界を説明する 1 箇所に寄せ、細かな selection-shape の比較は tests に委ねるのが current L2 では最も軽い。

## 8. Open questions

- machine-readable catalog asset を再検討する条件をどこで切るかは未決定。
- bundle manifest を導入するかは未決定。
- selector grammar / alias grammar の長期固定は未決定。
- path canonicalization policy は未決定。
- detached trace / audit serialization は未決定。
- richer host interface、multi-request scheduler、`Approximate` / `Compensate` の導入条件も未決定。

## 9. Suggested next prompt

current L2 named profile catalog を前提に、`crates/mir-semantics/src/harness.rs` の internal preset table と `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs` の public behavior coverage の対応が十分かを、internal tests / integration tests の責務分担に絞って点検してください。
