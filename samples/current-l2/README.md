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
- current matrix 自体は source target path までを固定し、reached stage inventory は下の ladder row に分けて持つ。

## current authored files

- actual source file として current repo にあるのは `e1-place-atomic-cut.txt`、`e2-try-fallback.txt`、`e21-try-atomic-cut-frontier.txt`、`e4-malformed-lineage.txt`、`e23-malformed-try-fallback-missing-fallback-body.txt` の current authored quintet である。
- `e3` は current matrix 上の target path を維持しつつ、remaining authored-row widen sequencing / guard line に置く。
- current lowerer first cut は single-line `require` / `ensure` と inline `admit` fragment を受け、multiline clause suite は fail-closed に止める。
- current runner first cut は accepted sample set 内の explicit path と sample stem shorthand を受け、host plan は explicit input に留める。

## current verification ladder

current reached-stage inventory は current authored quintet `e1` / `e2` / `e21` / `e4` / `e23` に付ける。

| sample stem | authored status | static gate | interpreter | formal hook | evidence route |
|---|---|---|---|---|---|
| `e1-place-atomic-cut` | `source-authored` | `reached(valid)` | `reached(explicit_failure)` | `reached(runtime_try_cut_cluster)` | `current_l2_source_sample_runner` + runtime detached bundle formal-hook smoke |
| `e2-try-fallback` | `source-authored` | `reached(valid)` | `reached(success)` | `reached(runtime_try_cut_cluster)` | `current_l2_source_sample_runner` + runtime detached bundle formal-hook smoke |
| `e21-try-atomic-cut-frontier` | `source-authored` | `reached(valid)` | `reached(success)` | `reached(runtime_try_cut_cluster)` | `current_l2_source_sample_runner` + runtime detached bundle formal-hook smoke |
| `e4-malformed-lineage` | `source-authored` | `reached(malformed)` | `not reached (static stop)` | `reached(fixture_static_cluster)` | `current_l2_source_sample_runner` + static-gate detached formal-hook smoke |
| `e23-malformed-try-fallback-missing-fallback-body` | `source-authored` | `reached(malformed)` | `not reached (static stop)` | `reached(fixture_static_cluster)` | `current_l2_source_sample_runner` + static-gate detached formal-hook smoke |
| `e3-option-admit-chain` | `source-target-only` | `not yet authored` | `not yet authored` | `not yet authored` | source target path only |
| `e21-try-atomic-cut-frontier` | `source-target-only` | `not yet authored` | `not yet authored` | `not yet authored` | source target path only |

- `formal hook` reached は current package では source-runner-native artifact ではなく、fixture-aligned detached artifact route を使う。
- `runtime_try_cut_cluster` と `fixture_static_cluster` は tool-neutral formal hook の current top であり、concrete theorem/model-check tool choice はまだ混ぜない。
- `source-target-only` は failure ではなく、current authored set 外の deferred authored row を意味する。
- current authoring / bless / regression flow は [.docs/current-l2-source-sample-authoring-policy.md](/home/yukatayu/dev/mir_poc_01/.docs/current-l2-source-sample-authoring-policy.md) と `python3 scripts/current_l2_source_sample_regression.py inventory|regression` を基準にする。
- theorem-first concrete pilot current cut では、tool-neutral formal hook artifact を入力にする `proof_notebook_review_unit` consumer を non-production helper/example に留める。

## non-goals

- final parser grammar の固定
- fixture JSON からの逆生成
- representative prose のそのまま昇格
- public CLI / exporter / backend path の先取り

## current authoring flow

- source sample を更新するときは source text / fixture mapping / matrix / ladder / snapshot docs を同じ task で揃える。
- `python3 scripts/current_l2_source_sample_regression.py inventory` は current authored quintet の file presence と deferred row absence を確認する。
- `python3 scripts/current_l2_source_sample_regression.py regression --run-label <label> --artifact-root <root>` は current authored quintet の lowering / runner / ladder / formal-hook smoke bundle を流す。
- current `bless` は retained artifact archive ではなく、review 済み repo-local sync と regression success の確認を意味する。

## next steps

- current next line は third widened row `e3-option-admit-chain` theorem-side / formal-hook guard comparison である
- その後に theorem-side plain bridge sketch actualization を置く
- compare-ready bridge sketch は plain bridge sketch の次段に残す
