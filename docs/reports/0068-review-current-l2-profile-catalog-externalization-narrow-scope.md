# Report 0068 — current l2 profile catalog externalization narrow scope review

- Date: 2026-04-01T23:38:08.942800Z
- Author / agent: Codex (GPT-5)
- Scope: current L2 named profile catalog externalization 比較文書に対する narrow scope review
- Decision levels touched: review only; no normative statement changed

## 1. Objective

`specs/examples/14-current-l2-profile-catalog-externalization.md` を中心とする docs-only change について、次の 3 点だけを確認する。

1. current L2 named profile catalog の semantics / implementation boundary を広げていないか
2. production manifest を既成事実にしていないか
3. unresolved を settled fact に書き換えていないか

## 2. Inputs consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/12-decision-register.md`
- `specs/examples/12-current-l2-selection-profiles.md`
- `specs/examples/13-current-l2-profile-catalog.md`
- `specs/examples/14-current-l2-profile-catalog-externalization.md`
- `crates/mir-semantics/src/harness.rs`
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`

## 3. Actions taken

- baseline docs を repository-required order で再確認した。
- `Documentation.md`、`specs/00-document-map.md`、`specs/10-open-questions.md`、`specs/examples/13-current-l2-profile-catalog.md` の差分を確認した。
- `specs/examples/14-current-l2-profile-catalog-externalization.md` を全文確認した。
- `ProfileCatalog`、`run_directory_profiled`、`run_directory_named_profile` の既存実装と targeted tests に照らして、新しい wording が existing boundary を越えていないかを照合した。
- `specs/12-decision-register.md` の `D-050` と見比べ、comparison wording が formal decision を上書きしていないかを確認した。

## 4. Files changed

- `docs/reports/0068-review-current-l2-profile-catalog-externalization-narrow-scope.md`

## 5. Commands run and exact outputs

- `git status --short`
  - `Documentation.md`、`specs/00-document-map.md`、`specs/10-open-questions.md`、`specs/examples/13-current-l2-profile-catalog.md` が modified、`specs/examples/14-current-l2-profile-catalog-externalization.md` と `docs/reports/0067-current-l2-profile-catalog-externalization-comparison.md` が untracked と読めた
- `git diff --stat`
  - tracked 4 files changed / 11 insertions / 2 deletions、new example file は untracked と読めた
- `git diff -- Documentation.md specs/00-document-map.md specs/10-open-questions.md specs/examples/13-current-l2-profile-catalog.md`
  - pointer update と `specs/10-open-questions.md` の 1 文追加だけであることを確認した
- `rg -n "ProfileCatalog|SelectionRequest|SelectionProfile|run_directory_profiled|smoke-runtime|runtime-e3|static-e4|smoke-static" crates/mir-semantics specs`
  - existing hard-coded alias resolver と four aliases の tests を確認した
- `sed` / `nl -ba` による line-level read
  - wording と implementation boundary の照合に使った

## 6. Evidence / findings

findings はなし。

evidence:

- 新しい comparison 文書は、selection / profile / batch / bundle の責務境界を変えず、catalog を alias resolver に留めると明記している。
- 実装側もその読みと一致しており、`ProfileCatalog::aliases()` / `ProfileCatalog::resolve()` は hard-coded resolver、`run_directory_named_profile()` は `run_directory_profiled()` の wrapper に留まる。
- comparison 文書は production manifest を導入した事実にしていない。machine-readable asset は comparison-only aid として繰り返し制限されている。
- externalization は unresolved のまま残されている。最も強い文も current L2 の comparison / 暫定運用選択に留まり、alias grammar、path canonicalization、revisit timing は unresolved のままである。
- `Documentation.md` と `specs/00-document-map.md` の更新は導線追加だけで、新しい semantics は足していない。

residual risk:

- comparison 文書の結論が将来 formal L2 decision として扱われるなら、`specs/12-decision-register.md` へ mirror が必要になる可能性はある。ただし今回の wording ではまだ comparison framing に留まっている。

## 7. Changes in understanding

- 現在の repository state は、実装・tests の両方で hard-coded named profile catalog をすでに体現している。
- 今回の文書は、その existing state を広げるのではなく、なぜ current L2 では externalization へ進まないかを比較根拠として補う companion である。
- したがって残る論点は semantics ではなく governance であり、comparison 結論を将来 decision register に昇格させるかどうかである。

## 8. Open questions

- 「4 alias 規模では hard-coded table が十分」という文を、companion / open question framing に留めるか、将来 `specs/12-decision-register.md` へ昇格させるか。
- comparison 文書内の `preset manifest` という語を今後も使うか、`machine-readable catalog asset` に寄せた方が production intent の過読を減らせるか。

## 9. Suggested next prompt

current L2 named profile catalog の externalization 比較を前提に、alias 定義の drift を減らすために `ProfileCatalog::aliases()` と `ProfileCatalog::resolve()` の二重定義をどう最小化するかを整理してください。hard-coded table は維持したまま、code / tests / docs の同期漏れだけを小さくする方向で検討してください。
