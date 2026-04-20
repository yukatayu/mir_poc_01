# 0848 — phase6 request-head clause bundle actualization

## Objective

Package 91 の
`perform` head / request clause suite bundle attachment compare floor を、
Package 89 / 90 の separate carrier minimum を崩さない thin wrapper として
`mir_ast::current_l2` に actualize し、
tests・docs・task snapshot を Package 92 先頭の reading へ同期する。

## Scope and assumptions

- `specs/` を規範正本として扱い、今回の追加は helper-local non-production parser carrier に留める。
- final parser grammar、final public parser / checker / runtime API、span-rich diagnostics、full `Program` lowering、final public verifier contract はこの task で固定しない。
- `Stage3RequestHeadClauseBundle` は thin wrapper first という compare-floor judgment を actualize するものであり、generic attachment frame や flattened head adoption ではない。
- `tasks.md` と `progress.md` は snapshot 文書として書き換え、Package 91 active の stale wording を残さない。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `specs/examples/562-current-l2-phase6-request-clause-suite-publicization-threshold-helper-mirror.md`
- `specs/examples/563-current-l2-phase6-perform-head-structural-carrier-threshold-helper-mirror.md`
- `specs/examples/564-current-l2-phase6-perform-head-request-clause-bundle-attachment-comparison.md`
- `crates/mir-ast/src/current_l2.rs`
- `crates/mir-ast/src/lib.rs`
- `crates/mir-ast/tests/current_l2_request_clause_suite_manifest.rs`
- `crates/mir-ast/tests/current_l2_perform_head_manifest.rs`
- `crates/mir-ast/tests/current_l2_stage3_request_clause_suite_spike.rs`
- `crates/mir-ast/tests/current_l2_stage3_perform_head_spike.rs`

## Actions taken

1. `Stage3RequestHeadClauseBundle` thin wrapper first という compare-floor judgment を、existing perform head parser / request clause suite parser reuse による actualization cut へ落とした。
2. `CurrentL2RequestHeadClauseBundleManifest`、`Stage3RequestAttachmentFrameKind`、`parse_stage3_request_head_clause_bundle_text()` を `crates/mir-ast/src/current_l2.rs` に追加した。
3. manifest test と spike test を追加し、fixture-backed positive pair、head-only bundle、perform-head failure reuse、request-clause-suite failure reuse を固定した。
4. `specs/examples/565` を追加し、Package 91 を compare floor のままではなく helper-local actualization close として source-backed に追えるようにした。
5. `Documentation.md`、`specs/00-document-map.md`、`specs/11-roadmap-and-workstreams.md`、`tasks.md`、`progress.md`、`plan/01`、`plan/11`、`plan/17`、`plan/18`、`plan/90` を更新し、current active line を Package 92 先頭へ進めた。

## Evidence / outputs / test results

- 追加ファイル:
  - `specs/examples/565-current-l2-phase6-perform-head-request-clause-bundle-thin-wrapper-threshold-helper-mirror.md`
  - `crates/mir-ast/tests/current_l2_request_head_clause_bundle_manifest.rs`
  - `crates/mir-ast/tests/current_l2_stage3_request_head_clause_bundle_spike.rs`
- 更新ファイル:
  - `crates/mir-ast/src/current_l2.rs`
  - `crates/mir-ast/src/lib.rs`
  - `Documentation.md`
  - `specs/00-document-map.md`
  - `specs/11-roadmap-and-workstreams.md`
  - `tasks.md`
  - `progress.md`
  - `plan/01-status-at-a-glance.md`
  - `plan/11-roadmap-near-term.md`
  - `plan/17-research-phases-and-autonomy-gates.md`
  - `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
  - `plan/90-source-traceability.md`
- 実行コマンドと結果:
  - `cargo test -p mir-ast --test current_l2_request_head_clause_bundle_manifest --test current_l2_stage3_request_head_clause_bundle_spike`
    - pass
  - `cargo test -p mir-ast --test current_l2_request_clause_suite_manifest --test current_l2_perform_head_manifest --test current_l2_stage3_request_clause_suite_spike --test current_l2_stage3_perform_head_spike`
    - pass

## What changed in understanding

- Package 91 は docs-only compare floor のまま残すより、thin wrapper actualization を helper-local carrier として先に閉じた方が adoption debt を減らせる段階に入っていた。
- `perform` head と request clause suite の responsibility split は維持したまま、combined carrier を narrow に actualize できることが確認できた。
- current live queue は Package 91 active ではなく、Package 92 finite-index strong typing first layer が自然な next line である。

## Open questions

- Package 92 で capture escape / cost bound rejected sample をどの current-L2 source sample path に置くか。
- Package 93 で Lean-first skeleton と generated stub corpus の接続を、どこまで code-generated helper に寄せるか。
- Package 94 で theorem-first bridge の preview row をどこまで code carrier に actualize するか。

## Suggested next prompt

Package 92 として finite decidable index fragment の first strong typing layer を actualize し、`p06 / p10 / p11 / p12` に capture escape rejected sample と simple cost rejected sample を加え、static carrier / sample / docs / Lean skeleton の 4 点を揃えてください。
