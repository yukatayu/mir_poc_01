# Report 0070 — current l2 profile alias mirror boundary

- Date: 2026-04-02T00:44:28.671855Z
- Author / agent: Codex (GPT-5)
- Scope: current L2 named profile catalog の hard-coded alias setを維持したまま、docs / tests / code の alias mirror をどこまで減らすべきかを整理し、最小の drift 削減だけを反映する。
- Decision levels touched: L2 の companion / verification helper 文書のみ。意味論本体は変更しない。

## 1. Objective

current L2 named profile catalog の alias mirror を減らす。特に次を目的とした。

- hard-coded named profile catalog は維持する
- `ProfileCatalog::aliases()` / `resolve()` の single source of truth を壊さない
- docs では alias 一覧の prose 列挙先を絞る
- tests では public behavior の machine-check を落とさずに重複記述を減らす
- machine-readable catalog asset / manifest には進まない

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
- `docs/reports/0063-current-l2-selection-helper.md`
- `docs/reports/0064-current-l2-selection-profiles.md`
- `docs/reports/0065-review-0064-short-rereview.md`
- `docs/reports/0066-current-l2-profile-catalog.md`
- `docs/reports/0067-current-l2-profile-catalog-externalization-comparison.md`
- `docs/reports/0068-review-current-l2-profile-catalog-externalization-narrow-scope.md`
- `docs/reports/0069-current-l2-profile-catalog-single-source-of-truth.md`
- `crates/mir-semantics/src/harness.rs`
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`

## 3. Actions taken

1. `code_mapper` で dirty state と alias mirror の散り方を棚卸しした。
2. code 側では alias 定義がすでに `crates/mir-semantics/src/harness.rs` の hard-coded preset table に集約されていることを確認した。
3. tests 側では alias ごとの `SelectionRequest` / selected bundle 期待値が複数 test に散っていたため、1 つの table-driven test に集約した。
4. ただし `resolved_request` の public behavior coverage を落とさないため、review 指摘後に `ProfileCatalog::resolve()` を test oracle として使う形は戻し、test 側の 1 箇所 table に literal expected request を残した。
5. docs 側では、具体的な alias 一覧を prose で列挙する正本を `specs/examples/13-current-l2-profile-catalog.md` に寄せ、`specs/examples/14-current-l2-profile-catalog-externalization.md`、`specs/10-open-questions.md`、`specs/12-decision-register.md` はその参照へ寄せた。
6. reviewer を narrow scope で 2 回使い、1 回目の finding を修正し、2 回目は `no findings` を確認した。

## 4. Files changed

- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
- `specs/10-open-questions.md`
- `specs/12-decision-register.md`
- `specs/examples/13-current-l2-profile-catalog.md`
- `specs/examples/14-current-l2-profile-catalog-externalization.md`
- `docs/reports/0070-current-l2-profile-alias-mirror-boundary.md`

## 5. Commands run and exact outputs

- `python3 scripts/new_report.py --slug current-l2-profile-alias-mirror-boundary`
  - Output: `/home/yukatayu/dev/mir_poc_01/docs/reports/0070-current-l2-profile-alias-mirror-boundary.md`
- `cargo check -p mir-semantics`
  - Output: `Finished \`dev\` profile [unoptimized + debuginfo] target(s) in 2.77s`
- `cargo test -p mir-semantics`
  - Output included:
    - `test harness::tests::named_profile_catalog_aliases_are_derived_from_internal_specs ... ok`
    - `test harness::tests::named_profile_catalog_resolve_is_derived_from_internal_specs ... ok`
    - `test named_profile_catalog_lists_expected_aliases ... ok`
    - `test run_directory_named_profiles_match_catalog_resolution_and_expected_selection ... ok`
    - `test run_directory_named_profile_rejects_unknown_alias ... ok`
    - `test result: ok. 30 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s`
- `python3 scripts/validate_docs.py`
  - Output at code/doc verification time: `Documentation scaffold looks complete.` / `Found 71 numbered report(s).`
- `git diff --check`
  - Output: なし
- `git commit -m "named profile alias mirror を最小化する"`
  - Output: GPG signing failed (`gpg: signing failed: No such file or directory`)
- `git commit --no-gpg-sign -m "named profile alias mirror を最小化する"`
  - Output: `[main c27e300] named profile alias mirror を最小化する`

## 6. Evidence / findings

- 作業開始時点の dirty state はなし。`code_mapper` の確認では `git status --short` は空だった。
- alias mirror の主な重複箇所は次だった。
  - code: `crates/mir-semantics/src/harness.rs` の hard-coded preset table
  - tests: `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
  - docs: `specs/examples/13-current-l2-profile-catalog.md`、`specs/examples/14-current-l2-profile-catalog-externalization.md`、`specs/10-open-questions.md`、`specs/12-decision-register.md`
- 1 回目の reviewer では medium finding が 1 件出た。
  - aggregated test が `summary.resolved_request == ProfileCatalog::resolve(alias).request` を使っており、public alias-to-request behavior の coverage を弱めていた。
  - これを、test 側の 1 箇所 table に literal expected request を持たせる形へ直して解消した。
- 2 回目の reviewer は `no findings` だった。
- 今回の最小整理で採った境界は次である。
  - code の single source of truth は既存 hard-coded preset table のまま
  - docs の具体的 alias 列挙は `specs/examples/13-current-l2-profile-catalog.md` に寄せる
  - tests は exact alias list と literal alias-to-request mapping を machine-check に残す
  - `must_explain` や長い運用説明は prose に残す
- 仕様本文コミットは次の 1 本である。
  - `c27e300` `named profile alias mirror を最小化する`
- この report 自身の commit hash は self-reference の都合で本文に固定していない。

## 7. Changes in understanding

- current L2 では、alias mirror を「完全に消す」よりも、「どこを machine-check に残し、どこを prose 正本 1 箇所に寄せるか」を切る方が自然だった。
- `resolved_request` は public behavior として exposed されているので、tests では catalog 実装を oracle に再利用しない方がよい。
- docs 側は、`specs/examples/13-current-l2-profile-catalog.md` を concrete alias list の唯一の prose 列挙先にし、他文書は導線化するだけで十分だった。

## 8. Open questions

- machine-readable catalog asset を再検討する条件は、引き続き `specs/examples/14-current-l2-profile-catalog-externalization.md` の比較条件に従って **未決定** のまま残す。
- bundle manifest を導入するか、selector grammar / alias grammar を長期固定するか、path canonicalization policy、detached trace / audit serialization、richer host interface、multi-request scheduler、`Approximate` / `Compensate` は今回も **未決定** のままである。

## 9. Suggested next prompt

current L2 named profile catalog を前提に、`specs/examples/13-current-l2-profile-catalog.md` の preset table と `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs` の named profile behavior table の責務分担を整理し、tests 側に残す literal expectation をどこまで helper 化してよいかを比較してください。
