# 565 — current L2 phase6-perform-head-request-clause-bundle thin-wrapper threshold helper mirror

## 目的

`specs/examples/562-current-l2-phase6-request-clause-suite-publicization-threshold-helper-mirror.md`、
`563-current-l2-phase6-perform-head-structural-carrier-threshold-helper-mirror.md`、
`564-current-l2-phase6-perform-head-request-clause-bundle-attachment-comparison.md`
で source-backed に積んだ
request-local clause suite と `perform` head の thin bundle line を、
`mir_ast::current_l2` の crate-local non-production carrier に narrow に映す。

ここで固定するのは
**current L2 phase6-perform-head-request-clause-bundle thin-wrapper minimum**
であり、

- span-rich diagnostics
- final grammar
- final public parser / checker / runtime surface
- full `Program` lowering

はまだ固定しない。

## helper mirror shape

```text
phase6_perform_head_request_clause_bundle_actualization = {
  carrier_kind = current_l2_nonproduction_request_head_clause_bundle,
  accepted_surface_refs = [
    stage3_perform_on_head_surface,
    stage3_perform_via_head_surface,
    stage3_request_clause_suite_surface
  ],
  code_anchor_refs = [
    mir_ast_current_l2_module,
    stage3_request_head_clause_bundle_tests
  ],
  retained_later_refs = [
    span_rich_diagnostics,
    final_grammar,
    final_public_parser_checker_runtime_surface,
    full_program_lowering
  ]
}

request_head_clause_bundle_bridge = {
  carrier = Stage3RequestHeadClauseBundle {
    perform_head,
    clause_suite,
    attachment_frame_kind = RequestLocalTwoSlotSuite
  },
  head_parser_reuse = parse_stage3_perform_head_text,
  clause_suite_parser_reuse = parse_stage3_request_clause_suite_text,
  failure_mode = fail_closed
}
```

## current actualization cut

- `crates/mir-ast/src/current_l2.rs` に
  - `CurrentL2RequestHeadClauseBundleManifest`
  - `current_l2_request_head_clause_bundle_manifest()`
  - `Stage3RequestAttachmentFrameKind`
  - `Stage3RequestHeadClauseBundle`
  - `parse_stage3_request_head_clause_bundle_text()`
  を置き、Package 89 / 90 で切った separate minimum を壊さず thin wrapper を crate 本体で inspectable にする。
- accepted surface は、
  - `perform op on target`
  - `perform op via chain_ref`
  - request-local fixed two-slot `require` / `ensure` suite
  に留める。
- attachment frame kind は `RequestLocalTwoSlotSuite` の single accepted cut に留め、
  generic attachment table や stringly frame へは widening しない。
- malformed guard として
  - missing `on` target
  - request-local unsupported direct child line
  を既存 parser reuse のまま drift なく保つ。
- head-only source は empty suite 付き thin bundle として許し、
  contract payload absent と attachment frame absent を混同しない。

## practical reading

current helper mirror が示すのは、次の 5 点だけである。

1. `perform` head と request clause suite の combined carrier は thin wrapper で actualize してよい。
2. bundle carrier は `Stage3RequestHeadClauseBundle { perform_head, clause_suite, attachment_frame_kind = RequestLocalTwoSlotSuite }` に留める。
3. perform head parser と request clause suite parser の separate responsibility は崩さず、bundle parser は再利用層としてのみ置く。
4. malformed input は fail-closed に保ち、head 側 / clause-suite 側の既存 guard wording をそのまま再利用する。
5. diagnostics / final grammar / final public parser-checker-runtime surface / full `Program` lowering にはまだ上げない。

## code anchors

- `crates/mir-ast/src/current_l2.rs`
- `crates/mir-ast/src/lib.rs`
- `crates/mir-ast/tests/current_l2_request_head_clause_bundle_manifest.rs`
- `crates/mir-ast/tests/current_l2_stage3_request_head_clause_bundle_spike.rs`
- `crates/mir-ast/tests/support/current_l2_stage3_predicate_fragment_spike_support.rs`

## stop line

- span_rich_diagnostics
- final_grammar
- final_public_parser_checker_runtime_surface
- full_program_lowering

## next promoted line

next promoted line は、
**Package 92 — first strong typing finite-index layer**
に置く。
