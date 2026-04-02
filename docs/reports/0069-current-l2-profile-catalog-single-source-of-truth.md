# Report 0069 — current l2 profile catalog single source of truth

- Date: 2026-04-02T00:10:18.994720Z
- Author / agent: Codex
- Scope: current L2 named profile catalog における alias drift 削減と single source of truth 化
- Decision levels touched: L2

## 1. Objective

current L2 では hard-coded named profile catalog を維持したまま、`ProfileCatalog::aliases()` と `ProfileCatalog::resolve()` の二重定義による drift を減らす。selection / profile / batch / bundle の既存責務境界は変えず、thin alias resolver のまま壊れにくくする。

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
- `crates/mir-semantics/src/harness.rs`
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`

## 3. Actions taken

- `code_mapper` で alias 定義の重複箇所を棚卸しし、`harness.rs` の `ProfileCatalog` 周辺に blast radius を閉じられることを確認した。
- `harness.rs` に unit test を先に追加し、まだ存在しない internal spec table に依存する RED を作った。
- `ProfileCatalog` の内部に `NamedProfileSpec` と single source of truth 用 macro を追加し、hard-coded preset table を 1 回だけ書けば alias slice と internal spec table の両方が生成される形にした。
- `ProfileCatalog::aliases()` は既存どおり `&'static [&'static str]` を返す public shape を維持しつつ、`resolve()` は同じ internal spec table から `SelectionProfile` を構築する形に寄せた。
- `SelectionProfile` builder ではなく `SelectionRequest` builder を internal spec table に持たせ、alias 文字列自体も table の外へ重複させないようにした。
- focused unit tests で、alias list と resolve 結果が internal spec table から導かれることを確認した。
- `specs/examples/13-current-l2-profile-catalog.md` に、hard-coded catalog 実装では alias list と alias→request 解決を 1 箇所の preset table から導いてよいことを 1 行追記した。
- reviewer を narrow scope で当て、最初の指摘だった `aliases()` の return type 変更を取り下げた後、fresh review で `no findings` を確認した。

## 4. Files changed

- `crates/mir-semantics/src/harness.rs`
- `specs/examples/13-current-l2-profile-catalog.md`
- `docs/reports/0069-current-l2-profile-catalog-single-source-of-truth.md`

開始時の既存 dirty state:

- なし

## 5. Commands run and exact outputs

1. `git status --short --branch`
   - `## main...origin/main [ahead 2]`
2. `cargo test -p mir-semantics named_profile_catalog_ -- --nocapture`
   - 初回 RED:
     - `error[E0425]: cannot find function 'named_profile_specs' in this scope`
   - 実装後 GREEN:
     - `test harness::tests::named_profile_catalog_aliases_are_derived_from_internal_specs ... ok`
     - `test harness::tests::named_profile_catalog_resolve_is_derived_from_internal_specs ... ok`
     - `test named_profile_catalog_lists_expected_aliases ... ok`
3. `cargo test -p mir-semantics run_directory_named_profile_ -- --nocapture`
   - `test run_directory_named_profile_rejects_unknown_alias ... ok`
   - `test run_directory_named_profile_runtime_e3_runs_only_e3_runtime_bundle ... ok`
   - `test run_directory_named_profile_smoke_runtime_matches_runtime_only_request ... ok`
   - `test run_directory_named_profile_smoke_static_matches_static_only_request ... ok`
   - `test run_directory_named_profile_static_e4_runs_only_e4_static_bundle ... ok`
4. `cargo check -p mir-semantics`
   - `Finished 'dev' profile [unoptimized + debuginfo] target(s) in 4.05s`
5. `cargo test -p mir-semantics`
   - `running 33 tests`
   - `test result: ok. 33 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s`
6. `python3 scripts/validate_docs.py`
   - `Documentation scaffold looks complete.`
   - `Found 69 numbered report(s).`
7. `git diff --check`
   - 無出力
8. `git commit --no-gpg-sign -m "named profile catalog の single source of truth 化を行う"`
   - `[main fee6647] named profile catalog の single source of truth 化を行う`

review evidence:

- initial narrow review は `aliases()` の public return type を `Vec` に変えた点だけを finding とした。
- その修正後、fresh narrow review は `no findings` だった。

## 6. Evidence / findings

主な evidence:

- `ProfileCatalog::aliases()` と `ProfileCatalog::resolve()` は、どちらも 1 つの hard-coded preset table から導かれるようになった。
- preset table は alias 名と `SelectionRequest` builder だけを持つので、alias 文字列と resolve 対応の重複を最小化できた。
- `run_directory_named_profile`、selection helper、profile helper、batch helper の責務は増えていない。
- unknown alias failure は従来どおり `InterpreterError::InvalidProgram("unknown named profile alias: ...")` のまま維持されている。
- `smoke-runtime`、`smoke-static`、`runtime-e3`、`static-e4` の runtime selection behavior も既存 integration tests で維持された。
- machine-readable catalog asset や manifest は導入していない。

判断:

- current L2 named profile catalog の drift は、internal preset table を single source of truth にし、public `aliases()` の slice boundary は維持する形が最小である。
- tests 側では、internal derivation の unit test と existing behavior の integration test を分けることで、single source of truth と public behavior の両方を確認できる。
- docs 側は machine-check に寄せず、current L2 で許す実装境界を 1 行だけ mirror するのが十分である。

関連 commit:

- `fee6647` `named profile catalog の single source of truth 化を行う`
- report 自身の commit hash は self-reference の都合で執筆時点では固定できないため、別 commit で記録する。

## 7. Changes in understanding

- drift の主因は alias 名そのものより、`aliases()` と `resolve()` が別々に hard-code されていた点だった。
- current L2 では externalization を持ち込まなくても、internal preset table と small unit test だけで drift をかなり減らせる。
- public API の shape を変えずに single source of truth 化する方が、current L2 の thin alias resolver という立場に合っている。

## 8. Open questions

- machine-readable catalog asset を再検討する条件は **未決定**。
- bundle manifest を導入するかどうかは **未決定**。
- selector grammar / alias grammar を長期固定するかどうかは **未決定**。
- path canonicalization policy は **未決定**。
- detached trace / audit serialization は **未決定**。
- richer host interface、multi-request scheduler、`Approximate` / `Compensate` は **未決定**。

## 9. Suggested next prompt

current L2 named profile catalog の single source of truth 化を前提に、docs / tests / code の alias mirror をどこまで減らすべきかを整理してください。semantics は変えず、alias 一覧の prose と machine-check の境界だけを見直す範囲に絞ると進めやすいです。
