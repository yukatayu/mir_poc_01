# Report 0077 — current L2 helper layer responsibility alignment

## Objective

current L2 の named profile catalog を前提に、`bundle loader` / `batch runner` / `selection helper` / `selection profile helper` / `named profile catalog` の各 helper layer について、**どこを public behavior として持ち、どこを下位 helper への thin delegation に留めるか**が同じ粒度で揃っているかを横断点検する。

## Scope and assumptions

- current L2 semantics 自体は変更しない。
- machine-readable catalog asset や manifest は導入しない。
- 対象は `specs/examples/09..13`、`crates/mir-semantics/src/harness.rs`、`crates/mir-semantics/tests/current_l2_minimal_interpreter.rs` の mirror / comment / test responsibility の整流に限る。
- 作業開始時点の worktree は clean だった。

## Documents consulted

1. `README.md`
2. `Documentation.md`
3. `specs/00-document-map.md`
4. `specs/01-charter-and-decision-levels.md`
5. `specs/02-system-overview.md`
6. `specs/03-layer-model.md`
7. `specs/04-mir-core.md`
8. `specs/09-invariants-and-constraints.md`
9. `specs/10-open-questions.md`
10. `specs/11-roadmap-and-workstreams.md`
11. `specs/12-decision-register.md`
12. `specs/examples/09-current-l2-bundle-loader.md`
13. `specs/examples/10-current-l2-batch-runner.md`
14. `specs/examples/11-current-l2-selection-helper.md`
15. `specs/examples/12-current-l2-selection-profiles.md`
16. `specs/examples/13-current-l2-profile-catalog.md`
17. `specs/examples/14-current-l2-profile-catalog-externalization.md`
18. `docs/reports/0071-current-l2-profile-catalog-docs-tests-code-boundary.md`
19. `docs/reports/0072-review-helper-boundary-profile-catalog.md`
20. `docs/reports/0073-current-l2-named-profile-test-helper-boundary.md`
21. `docs/reports/0074-current-l2-profile-catalog-test-responsibility-boundary.md`
22. `docs/reports/0075-current-l2-profile-catalog-followup-review.md`
23. `docs/reports/0076-current-l2-profile-catalog-internal-integration-test-boundary.md`
24. `crates/mir-semantics/src/harness.rs`
25. `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`

## Actions taken

1. helper stack の実コードを読み、実際の call stack を `run_directory_named_profile -> run_directory_profiled -> select_bundles_from_request -> batch_summary_from_discovery -> run_bundle` として再確認した。
2. `specs/examples/09..13` の prose を比較し、layer ごとの責務記述が次の 2 軸で同じ粒度になっているかを確認した。
   - layer 自身が public behavior として持つもの
   - 下位 helper への thin delegation に留めるもの
3. `specs/examples/09..13` に最小限の wording を追加し、bundle / batch / selection / profile / catalog の各層で上記 2 軸を明示した。
4. `crates/mir-semantics/src/harness.rs` の `run_directory_profiled` / `run_directory_named_profile` に薄い wrapper であることを明記する comment を追加した。
5. `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs` に、profile-layer integration tests が selected counts / concrete fixture shape を持ち、named-profile integration tests は alias / `resolved_request` / unknown alias failure / thin delegation を持つことを明記する comment を追加した。
6. narrow-scope reviewer を実行し、差分が semantics を変えず、public behavior coverage を弱めていないことを確認した。

## Files changed

- `specs/examples/09-current-l2-bundle-loader.md`
- `specs/examples/10-current-l2-batch-runner.md`
- `specs/examples/11-current-l2-selection-helper.md`
- `specs/examples/12-current-l2-selection-profiles.md`
- `specs/examples/13-current-l2-profile-catalog.md`
- `crates/mir-semantics/src/harness.rs`
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`

## Commands run

```bash
git status --short --branch
rg -n "thin delegation|public behavior|resolved_request|selected bundle|fixture suffix|unknown alias|named profile|selection helper|profile helper|batch runner|bundle loader" specs/examples/09-current-l2-bundle-loader.md specs/examples/10-current-l2-batch-runner.md specs/examples/11-current-l2-selection-helper.md specs/examples/12-current-l2-selection-profiles.md specs/examples/13-current-l2-profile-catalog.md crates/mir-semantics/src/harness.rs crates/mir-semantics/tests/current_l2_minimal_interpreter.rs
git diff -- specs/examples/09-current-l2-bundle-loader.md specs/examples/10-current-l2-batch-runner.md specs/examples/11-current-l2-selection-helper.md specs/examples/12-current-l2-selection-profiles.md specs/examples/13-current-l2-profile-catalog.md crates/mir-semantics/src/harness.rs crates/mir-semantics/tests/current_l2_minimal_interpreter.rs
git diff --check
cargo test -p mir-semantics
python3 scripts/validate_docs.py
git add specs/examples/09-current-l2-bundle-loader.md specs/examples/10-current-l2-batch-runner.md specs/examples/11-current-l2-selection-helper.md specs/examples/12-current-l2-selection-profiles.md specs/examples/13-current-l2-profile-catalog.md crates/mir-semantics/src/harness.rs crates/mir-semantics/tests/current_l2_minimal_interpreter.rs
git commit --no-gpg-sign -m "helper layer の責務境界を横断整流する"
```

## Evidence / outputs / test results

- `cargo test -p mir-semantics`
  - unit tests 2 件 pass
  - integration tests 32 件 pass
  - doc tests 0 件
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 76 numbered report(s).`
- `git diff --check`
  - 無出力
- reviewer
  - `No findings.`
  - 差分は semantics を変えず、`run_directory_profiled` と `run_directory_named_profile` の thin wrapper 性を保っているとの確認を得た。

## What changed in understanding

- bundle loader は helper stack の最下位で、bundle 単位の expectation compare を public behavior として持つ。
- batch runner は directory discovery / classification / aggregate summary を public behavior として持ち、per-bundle compare 自体は bundle helper の結果を集約する。
- selection helper は primitive filter と selected summary を public behavior として持ち、複合 request や alias 解決は持たない。
- selection profile helper は composed request と profile 名付き summary を public behavior として持ち、selected counts / concrete fixture shape の coverage はこの層の integration tests に残してよい。
- named profile catalog は alias 一覧、literal `resolved_request`、unknown alias failure、profiled execution への thin delegation を public behavior として持ち、selected counts / concrete fixture suffix の詳細比較は下位 profile layer へ留める。

## Open questions

- `host plan coverage failure` は現在 batch summary 側で error text から分類しており、typed な bundle-level outcome としてはまだ分離されていない。これは既存仕様を壊さないが、将来の drift 点としては残る。
- machine-readable catalog asset や manifest を再検討する条件は依然として **未決定** である。
- selector grammar / alias grammar の長期固定、path canonicalization policy、detached trace / audit serialization、richer host interface、multi-request scheduler、`Approximate` / `Compensate` は引き続き **未決定** である。

## Commits

- 仕様本文と code / test comment の整流: `11cb32d` `helper layer の責務境界を横断整流する`
- この report 自身の commit hash は self-reference の都合で本文に固定しない。

## Suggested next prompt

`current L2 helper stack について、bundle / batch / selection / profile / named-profile の summary shape が将来 typed API に昇格するとき、どの failure kind だけを first-class field にし、どれを現状どおり aggregate report に留めるべきかを比較整理してください。`
