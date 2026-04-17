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

を review 済みの repo-local sync に揃え、`inventory` / `regression` を通し、必要なら emitted review-unit / model-check carrier helper output を inspect することである。

## current scope

- current authored source sample は `e1` / `e2` / `e3` / `e4` / `e14` / `e15` / `e16` / `e13` / `e19` / `e21` / `e22` / `e18` / `e20` / `e23` の authored fourteen に置く。
- corrected runnable prototype sample は `samples/prototype/` に置き、この current authored inventory へ混ぜない。
- exact rough stimulus preservation は `samples/not_implemented/` に置き、current parser / runner 対象にしない。
- `e3` は source-authored row まで actualize 済みだが、formal hook stage は `not reached (guarded)` に留め、current theorem-side consumer と current formal-hook top は widen しない。
- post-sextet first cluster は `e21` / `e22` try-rollback locality contrast として actualize 済みであり、stable-static edge-pair first reopen では existing `e4` row と new `e19` row を source-backed static-stop pair へ actualize 済みである。さらに capability second source-backed widening actualization では `e13/e20` pair も source-authored static-stop pair として actualize 済みであり、duplicate-cluster source-authored static-stop pair actualization では `e14/e15` pair も source sample / lowerer / runner / ladder / emitted artifact wiring / regression helper まで widened している。public operational CLI concrete shell actualization と shared-space room-profile / host-binding bridge-only note も fixed 済みであり、repo-level current line は theory-lab lane の modality internalization trigger note と reserve integration lane の packaging / fairness-replay reserve に進んでいる。
- regression helper は `python3 scripts/current_l2_source_sample_regression.py` を使う。
- public CLI、retained artifact bless/update、fixture JSON からの逆生成は current scope 外である。
- runnable sample 冒頭の leading contiguous `#` line block は current authoring convenience として許してよい。

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
- `cargo test -p mir-runtime --test current_l2_source_sample_emitted_artifact_wiring`
- `cargo test -p mir-semantics --test current_l2_formal_hook_support`
- `python3 scripts/current_l2_detached_loop.py smoke-formal-hook-runtime e1-place-atomic-cut ...`
- `python3 scripts/current_l2_detached_loop.py smoke-formal-hook-runtime e2-try-fallback ...`
- `python3 scripts/current_l2_detached_loop.py smoke-formal-hook-runtime e21-try-atomic-cut-frontier ...`
- `python3 scripts/current_l2_detached_loop.py smoke-formal-hook-runtime e22-try-atomic-cut-place-mismatch ...`
- `python3 scripts/current_l2_detached_loop.py smoke-formal-hook-static e4-malformed-lineage ...`
- `python3 scripts/current_l2_detached_loop.py smoke-formal-hook-static e14-malformed-duplicate-option-declaration ...`
- `python3 scripts/current_l2_detached_loop.py smoke-formal-hook-static e15-malformed-duplicate-chain-declaration ...`
- `python3 scripts/current_l2_detached_loop.py smoke-formal-hook-static e16-malformed-missing-chain-head-option ...`
- `python3 scripts/current_l2_detached_loop.py smoke-formal-hook-static e13-malformed-capability-strengthening ...`
- `python3 scripts/current_l2_detached_loop.py smoke-formal-hook-static e19-malformed-target-mismatch ...`
- `python3 scripts/current_l2_detached_loop.py smoke-formal-hook-static e18-malformed-missing-successor-option ...`
- `python3 scripts/current_l2_detached_loop.py smoke-formal-hook-static e20-malformed-late-capability-strengthening ...`
- `python3 scripts/current_l2_detached_loop.py smoke-formal-hook-static e23-malformed-try-fallback-missing-fallback-body ...`

helper は current authored fourteen を inventory 対象にする。ただし `e3` は current formal-hook top の外なので、formal-hook smoke sub-bundleには加えない。
`inventory` は mismatch があれば non-zero で止まり、`regression` も先に同じ inventory check を行う。

## current non-goals

- public runner CLI
- source sample と fixture JSON の自動相互変換
- retained detached artifact bless / archive policy
- theorem/model-check concrete tool binding
