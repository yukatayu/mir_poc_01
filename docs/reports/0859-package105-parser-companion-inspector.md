# 0859 — Package 105 parser companion inspector

## Objective

Package 105 として、representative parser companion sample `p06 / p07 / p08` の
parse result を final public parser API に上げずに repo-local command で inspectable にする。

## Scope and assumptions

- target は `Stage3RequestHeadClauseBundle` と representative slice `p06 / p07 / p08` に限定する。
- inspection は helper-local / non-production の readable cut に留める。
- final grammar、final public parser / checker / runtime API、full `Program` lowering、final diagnostics contract は still later に残す。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/examples/577-current-l2-parser-side-companion-surface-bundle-actualization.md`
- `specs/examples/578-current-l2-parser-side-bundle-to-helper-bridge-actualization.md`
- `samples/prototype/current-l2-parser-companion/README.md`
- `plan/11-roadmap-near-term.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`

## Actions taken

1. failing test `crates/mir-ast/tests/current_l2_request_head_clause_bundle_inspector_manifest.rs` を追加し、inspector manifest と representative sample `p06 / p07 / p08` の JSON / pretty rendering 期待値を先に固定した。
2. `crates/mir-ast/src/current_l2.rs` に inspector manifest、inspection struct、inspection / render 関数を追加した。
3. `crates/mir-ast/examples/current_l2_inspect_request_head_clause_bundle.rs` を追加し、companion sample path を受け取って JSON / pretty summary を出す repo-local command を actualize した。
4. `samples/prototype/current-l2-parser-companion/README.md` に inspector command を追記した。
5. `specs/examples/579` を追加し、snapshot / roadmap / queue を Package 106 へ同期した。

## Files changed

- `crates/mir-ast/src/current_l2.rs`
- `crates/mir-ast/src/lib.rs`
- `crates/mir-ast/examples/current_l2_inspect_request_head_clause_bundle.rs`
- `crates/mir-ast/tests/current_l2_request_head_clause_bundle_inspector_manifest.rs`
- `samples/prototype/current-l2-parser-companion/README.md`
- `specs/examples/579-current-l2-parser-side-request-head-clause-bundle-inspector-actualization.md`

## Commands run

- `cargo test -p mir-ast --test current_l2_request_head_clause_bundle_inspector_manifest`
- `cargo run -q -p mir-ast --example current_l2_inspect_request_head_clause_bundle -- samples/prototype/current-l2-parser-companion/p06-typed-proof-owner-handoff.request.txt --format json`
- `cargo run -q -p mir-ast --example current_l2_inspect_request_head_clause_bundle -- samples/prototype/current-l2-parser-companion/p07-dice-late-join-visible-history.request.txt --format pretty`

## Evidence / outputs / test results

- `cargo test -p mir-ast --test current_l2_request_head_clause_bundle_inspector_manifest`
  - 2 tests passed
- JSON output
  - `p06` で `op = prove_owner_handoff`、`target_kind = via`、`target_ref = review_unit`、`require = typed_guard`、`ensure = owner_is(next_owner)` を確認
- pretty output
  - `p07` で `target: on authoritative_room`、`ensure: history_visible_as_past` を確認

## What changed in understanding

- parser-side carrier は parse test だけでも十分だったが、representative sample の parse result 自体を repo-local command で見せると、companion surface の役割差がより誤読されにくくなった。
- mapping matrix をまだ持たない段階でも、path-first inspector を先に actualize することで Package 106 の対象を narrow に保てる。

## Open questions

- inspector command に sample id shorthand を持たせるか、path-first のままにするか。
- reserve / negative sample まで inspection target を widen する順番をどうするか。

## Suggested next prompt

Package 106 として、original prototype / parser companion / guided bundle / Lean artifact / anchor spec-report の mapping matrix を docs / helper / traceability に actualize してください。final public parser API へは上げないでください。
