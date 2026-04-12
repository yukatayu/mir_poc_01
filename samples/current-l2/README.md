# current-l2 source samples

この directory は、current L2 fixed subset の **source text sample corpus** を置く repo-root layer である。

## 位置づけ

- representative prose (`specs/examples/00...01`) でもない。
- machine-readable fixture corpus (`crates/mir-ast/tests/fixtures/current-l2/`) でもない。
- fixed subset の syntax-backed regression / lowering / runner / verification ladder に進むための第 3 層である。

## current policy

- directory は `samples/current-l2/` に固定する。
- 初期 corpus は flat layout にする。
- extension は final grammar を意味しない neutral text file として `.txt` を使う。
- sample stem は fixture stem / `fixture_id` と揃える。
- filename に verdict、parser tranche、reached stage は埋め込まない。

## initial cluster

current first cluster は次を想定する。

- `e1-place-atomic-cut.txt`
- `e2-try-fallback.txt`
- `e3-option-admit-chain.txt`
- `e4-malformed-lineage.txt`
- `e21-try-atomic-cut-frontier.txt`
- `e23-malformed-try-fallback-missing-fallback-body.txt`

## current mapping matrix

current first matrix は、representative prose / fixture corpus / source target を次の row で結ぶ。

| order | sample stem | representative anchor | status | fixture id | mode | source target | coverage focus | expected static | expected runtime |
|---|---|---|---|---|---|---|---|---|---|
| 1 | `e1-place-atomic-cut` | `E1` | `direct` | `e1_place_atomic_cut` | `runtime_fixture` | `samples/current-l2/e1-place-atomic-cut.txt` | post-cut failure でも pre-cut mutation を rollback しない | `valid` | `explicit_failure` |
| 2 | `e2-try-fallback` | `E2` | `direct` | `e2_try_fallback` | `runtime_fixture` | `samples/current-l2/e2-try-fallback.txt` | local rollback 後に explicit fallback branch が走る | `valid` | `success` |
| 3 | `e3-option-admit-chain` | `E3-variant` | `variant` | `e3_option_admit_chain` | `runtime_fixture` | `samples/current-l2/e3-option-admit-chain.txt` | option-local `admit` miss を non-admissible skip に留める | `valid` | `success` |
| 4 | `e4-malformed-lineage` | `E4` | `direct` | `e4_malformed_lineage` | `static_only_fixture` | `samples/current-l2/e4-malformed-lineage.txt` | edge-local lineage mismatch を static stop に留める | `malformed` | `not_evaluated` |
| 5 | `e21-try-atomic-cut-frontier` | `E21` | `direct` | `e21_try_atomic_cut_frontier` | `runtime_fixture` | `samples/current-l2/e21-try-atomic-cut-frontier.txt` | try body 内 `atomic_cut` の frontier update | `valid` | `success` |
| 6 | `e23-malformed-try-fallback-missing-fallback-body` | unresolved representative anchor | `unresolved` | `e23_malformed_try_fallback_missing_fallback_body` | `static_only_fixture` | `samples/current-l2/e23-malformed-try-fallback-missing-fallback-body.txt` | empty `fallback_body` structural malformed floor | `malformed` | `not_evaluated` |

## current notes

- `e3` は plain `E3` ではなく fixture-side `source_example_id = E3-variant` を mirror する。
- `e23` は fixture-side `source_example_id = E23` を already 持つが、current representative prose row はまだない。
- current matrix では source target path だけを固定し、actual reached stage や bless policy はまだ埋め込まない。

## current authored files

- actual source file として current repo にあるのは `e4-malformed-lineage.txt`、`e2-try-fallback.txt`、`e23-malformed-try-fallback-missing-fallback-body.txt` の first trio である。
- `e1` / `e3` / `e21` は current matrix 上の target path を維持しつつ、runner / ladder task で actualization 順を決める。
- current lowerer first cut は single-line `require` / `ensure` と inline `admit` fragment を受け、multiline clause suite は fail-closed に止める。

## non-goals

- final parser grammar の固定
- fixture JSON からの逆生成
- representative prose のそのまま昇格
- public CLI / exporter / backend path の先取り

## next steps

- syntax-backed runner と verification ladder へ接続する
- source-sample authoring / bless policy を narrow に閉じる
