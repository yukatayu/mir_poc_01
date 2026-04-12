# current L2 source sample authoring policy

## 目的

この文書は、Phase 6 current L2 fixed subset の source text sample を追加・更新するときの
**repo-local authoring / bless / regression flow**
を narrow に固定する。

ここでの `bless` は final public command や retained artifact archive を意味しない。
current meaning は、

- source text
- fixture mapping
- README matrix / ladder row
- snapshot docs

を review 済みの repo-local sync に揃え、current regression bundle を通すことである。

## current scope

- current authored source sample は `e1` / `e2` / `e3` / `e21` / `e22` / `e4` / `e23` の authored septet に置く。
- `e3` は source-authored row まで actualize 済みだが、formal hook stage は `not reached (guarded)` に留め、current theorem-side consumer と current formal-hook top は widen しない。
- post-sextet first cluster は `e21` / `e22` try-rollback locality contrast として actualize 済みであり、next line は stable static malformed post-contrast sequencing に置く。
- regression helper は `python3 scripts/current_l2_source_sample_regression.py` を使う。
- public CLI、retained artifact bless/update、fixture JSON からの逆生成は current scope 外である。

## authoring steps

1. sample が current authored row か deferred target-only row かを先に決める。
2. authored row を更新するなら `samples/current-l2/<stem>.txt` を編集する。
3. fixture corpus 側の対応 row と expectation が still aligned か確認する。
4. `samples/current-l2/README.md` の mapping matrix と verification ladder row を更新する。
5. current task に応じて `Documentation.md`、`progress.md`、`tasks.md`、relevant `plan/`、research abstract を mirror 更新する。
6. `python3 scripts/current_l2_source_sample_regression.py inventory` で current authored/deferred inventory と file presence / absence を確認する。
7. `python3 scripts/current_l2_source_sample_regression.py regression --run-label <label> --artifact-root <root>` を実行する。
8. 非自明 task なら report を追加し、current package / next line を snapshot に反映する。

## regression bundle

current regression helper が束ねるのは次である。

- `cargo test -p mir-runtime --test current_l2_source_lowering`
- `cargo test -p mir-runtime --test current_l2_source_sample_runner`
- `cargo test -p mir-runtime --test current_l2_source_sample_verification_ladder`
- `cargo test -p mir-semantics --test current_l2_formal_hook_support`
- `python3 scripts/current_l2_detached_loop.py smoke-formal-hook-runtime e1-place-atomic-cut ...`
- `python3 scripts/current_l2_detached_loop.py smoke-formal-hook-runtime e2-try-fallback ...`
- `python3 scripts/current_l2_detached_loop.py smoke-formal-hook-runtime e21-try-atomic-cut-frontier ...`
- `python3 scripts/current_l2_detached_loop.py smoke-formal-hook-runtime e22-try-atomic-cut-place-mismatch ...`
- `python3 scripts/current_l2_detached_loop.py smoke-formal-hook-static e4-malformed-lineage ...`
- `python3 scripts/current_l2_detached_loop.py smoke-formal-hook-static e23-malformed-try-fallback-missing-fallback-body ...`

helper は current authored septet を inventory 対象にする。ただし `e3` は current formal-hook top の外なので、formal-hook smoke sub-bundleには加えない。
`inventory` は mismatch があれば non-zero で止まり、`regression` も先に同じ inventory check を行う。

## current non-goals

- public runner CLI
- source sample と fixture JSON の自動相互変換
- retained detached artifact bless / archive policy
- theorem/model-check concrete tool binding
