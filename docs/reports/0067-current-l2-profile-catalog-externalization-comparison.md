# Report 0067 — current l2 profile catalog externalization comparison

- Date: 2026-04-01T23:33:32.429083Z
- Author / agent: Codex
- Scope: current L2 named profile catalog を hard-coded table に留めるか、machine-readable catalog asset / preset manifest へ外出しするかの比較整理
- Decision levels touched: L2

## 1. Objective

`smoke-runtime`、`smoke-static`、`runtime-e3`、`static-e4` のような current L2 named profile alias について、現状の hard-coded table が最小か、それとも小さな machine-readable asset へ外出しする価値があるかを比較整理する。production manifest には進まず、PoC 実験ループにとっての最小判断だけを記録する。

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
- `specs/examples/13-current-l2-profile-catalog.md`
- `docs/reports/0061-review-0060-short-rereview.md`
- `docs/reports/0062-current-l2-batch-runner.md`
- `docs/reports/0063-current-l2-selection-helper.md`
- `docs/reports/0064-current-l2-selection-profiles.md`
- `docs/reports/0065-review-0064-short-rereview.md`
- `docs/reports/0066-current-l2-profile-catalog.md`
- `crates/mir-semantics/src/harness.rs`
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`

## 3. Actions taken

- `0066` に対する short re-review を実施し、`no findings` を確認した。
- hard-coded named profile catalog の current shape を再確認し、`ProfileCatalog::aliases()` / `ProfileCatalog::resolve()` が thin alias resolver に留まっていることを確認した。
- machine-readable catalog asset を導入する場合の最小案を、production manifest ではなく comparison-only な preset table として整理した。
- hard-coded table 維持案と machine-readable preset table 案を、PoC 実験ループ、helper の責務境界、grammar 固定圧、path canonicalization へのにじみ、tests / review のしやすさで比較した。
- `specs/examples/14-current-l2-profile-catalog-externalization.md` を新設し、comparison 結果を補助文書として記録した。
- `specs/examples/13-current-l2-profile-catalog.md` に 14 への導線を追加した。
- `Documentation.md`、`specs/00-document-map.md`、`specs/10-open-questions.md` を最小更新し、current L2 では現状の 4 alias 規模なら hard-coded table が最小であること、externalization は comparison 対象に留めることを明記した。
- final narrow review を実施し、`no findings` を確認した。

## 4. Files changed

- `Documentation.md`
- `specs/00-document-map.md`
- `specs/10-open-questions.md`
- `specs/examples/13-current-l2-profile-catalog.md`
- `specs/examples/14-current-l2-profile-catalog-externalization.md`
- `docs/reports/0067-current-l2-profile-catalog-externalization-comparison.md`
- `docs/reports/0068-review-current-l2-profile-catalog-externalization-narrow-scope.md`

開始時の既存 dirty state:

- なし

## 5. Commands run and exact outputs

1. `python3 scripts/new_report.py --slug current-l2-profile-catalog-externalization-comparison`
   - `/home/yukatayu/dev/mir_poc_01/docs/reports/0067-current-l2-profile-catalog-externalization-comparison.md`
2. `git status --short --branch`
   - `## main...origin/main`
3. `cargo test -p mir-semantics named_profile_catalog_lists_expected_aliases run_directory_named_profile_runtime_e3_runs_only_e3_runtime_bundle -- --nocapture`
   - `error: unexpected argument 'run_directory_named_profile_runtime_e3_runs_only_e3_runtime_bundle' found`
4. `cargo test -p mir-semantics named_profile_catalog_lists_expected_aliases -- --nocapture`
   - `test named_profile_catalog_lists_expected_aliases ... ok`
   - `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 32 filtered out; finished in 0.00s`
5. `cargo test -p mir-semantics run_directory_named_profile_runtime_e3_runs_only_e3_runtime_bundle -- --nocapture`
   - `test run_directory_named_profile_runtime_e3_runs_only_e3_runtime_bundle ... ok`
   - `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 32 filtered out; finished in 0.01s`
6. `python3 scripts/validate_docs.py`
   - `Documentation scaffold looks complete.`
   - `Found 67 numbered report(s).`
7. `git diff --check`
   - 無出力
8. `git commit --no-gpg-sign -m "current L2 の profile catalog externalization 比較を整理する"`
   - `[main 90a7a04] current L2 の profile catalog externalization 比較を整理する`
   - ` 5 files changed, 115 insertions(+), 2 deletions(-)`
   - ` create mode 100644 specs/examples/14-current-l2-profile-catalog-externalization.md`

## 6. Evidence / findings

主な evidence:

- `0066` の short re-review は `no findings` で、current L2 named profile catalog は thin alias resolver の境界に留まっていることを再確認できた。
- 現行実装では alias は 4 個だけで、`ProfileCatalog::aliases()` と `ProfileCatalog::resolve()` の hard-coded table でも PoC 実験ループ上の負担はまだ小さい。
- machine-readable preset table 案は、preset 差し替え実験をコード変更なしで回しやすくする利点がある一方で、loader / validation / missing asset handling を新たに導入し、catalog layer に pseudo-manifest 的な責務を持ち込みやすい。
- current L2 で machine-readable asset を昇格させると、alias grammar / selector grammar / path canonicalization / file placement に対する固定圧が早く立つ。
- 現状の 4 alias 規模では、asset 化の利点より hard-coded table の単純さの方が勝る。
- final narrow review も `no findings` で、production manifest を既成事実化しておらず、unresolved も unresolved のまま保たれていることを確認した。

判断:

- current L2 では hard-coded named profile catalog のままで十分である。
- machine-readable catalog asset は comparison 対象として意味はあるが、現時点では production でも current L2 helper でもなく、再検討条件付きの future option として残すのが最小である。
- 再検討条件は少なくとも次である。
  - alias 数が small named catalog と言いにくい規模まで増える
  - preset を Rust 実装の変更なしで頻繁に差し替えたい
  - preset table 自体を asset review / validation の対象にしたい
  - 複数 preset set を試験的に切り替える要求が増える

関連 commit:

- `90a7a04` `current L2 の profile catalog externalization 比較を整理する`
- report 自身の commit hash は self-reference の都合で執筆時点では固定できないため、追記 commit を別に記録する。

## 7. Changes in understanding

- named profile catalog の externalization は、単に alias table を file に出すだけでは済まず、loader / schema / placement policy を伴うため、current L2 では helper 層の責務を重くしやすいことが見えた。
- current L2 で重要なのは「asset 化できるか」より「asset 化が existing helper boundary を押し広げないか」であり、現状はその押し広げコストの方が大きい。
- したがって、今の段階では hard-coded table 維持を最小選択とし、asset externalization は将来の比較条件が揃ったときにだけ再検討するのが自然だと整理できた。

## 8. Open questions

- bundle manifest を導入するかどうかは **未決定**。
- alias grammar / selector grammar を長期固定するかどうかは **未決定**。
- path canonicalization policy は **未決定**。
- detached trace / audit serialization は **未決定**。
- richer host interface、multi-request scheduler、`Approximate` / `Compensate` は **未決定**。
- named profile catalog の externalization をどの条件で再検討するかは、現時点では companion docs の comparison 基準に留め、decision register に昇格させるかは **未決定**。

## 9. Suggested next prompt

current L2 named profile catalog の externalization 比較を前提に、alias 定義の drift を減らすために `ProfileCatalog::aliases()` と `ProfileCatalog::resolve()` の二重定義をどう最小化するかを整理してください。hard-coded table は維持したまま、code / tests / docs の同期漏れだけを小さくする方向で検討してください。
