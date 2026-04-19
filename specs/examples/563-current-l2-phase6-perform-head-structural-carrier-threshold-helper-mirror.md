# 563 — current L2 phase6-perform-head structural carrier threshold helper mirror

## 目的

`specs/examples/299-current-l2-phase5-proof-protocol-runtime-policy-handoff-closeout-ready-phase6-actual-parser-ast-carrier-first-tranche-comparison.md`、
`305-current-l2-phase6-compile-ready-checkpoint-close-ready-phase6-next-reopen-sequencing-comparison.md`、
`313-current-l2-phase6-parser-side-follow-up-package-sequencing-ready-phase6-parser-second-tranche-shared-single-attachment-frame-first-package-comparison.md`、
`562-current-l2-phase6-request-clause-suite-publicization-threshold-helper-mirror.md`
で source-backed に積んだ
`perform` head line を、
`mir_ast::current_l2` の crate-local non-production carrier に narrow に映す。

ここで固定するのは
**current L2 phase6-perform-head structural carrier minimum**
であり、

- request clause suite bundle attachment
- span-rich diagnostics
- final grammar
- final public parser / checker / runtime surface

はまだ固定しない。

## helper mirror shape

```text
phase6_perform_head_structural_carrier = {
  carrier_kind = current_l2_nonproduction_perform_head_carrier,
  accepted_surface_refs = [
    stage3_perform_owner_surface,
    stage3_perform_on_head_surface,
    stage3_perform_via_head_surface
  ],
  code_anchor_refs = [
    mir_ast_current_l2_module,
    stage3_perform_head_tests
  ],
  retained_later_refs = [
    request_clause_suite_bundle_attachment,
    span_rich_diagnostics,
    final_grammar
  ]
}

perform_head_bridge = {
  carrier = Stage3PerformHead {
    op,
    target_ref = On(target) | Via(chain_ref)
  },
  owner = perform,
  multiplicity = single_head_line,
  termination = head_line_only
}
```

## current actualization cut

- `crates/mir-ast/src/current_l2.rs` に
  - `CurrentL2PerformHeadManifest`
  - `current_l2_perform_head_manifest()`
  - `Stage3PerformTargetRef`
  - `Stage3PerformHead`
  - `parse_stage3_perform_head_text()`
  を置き、perform head line の minimum を crate 本体で inspectable にする。
- accepted surface は、
  - owner keyword `perform`
  - `perform op on target`
  - `perform op via chain_ref`
  に留める。
- current helper は request clause suite carrier を prerequisite / entry criteria として前提にするが、
  contract payload 自体はまだ perform head carrier へ混ぜない。
- malformed guard として
  - missing `on` target
  - missing `via` chain ref
  - unsupported channel keyword
  - extra token after target
  を current head carrier と一緒に drift なく保つ。

## practical reading

current helper mirror が示すのは、次の 5 点だけである。

1. `perform` head の owner / op / target-or-via shape は non-production parser carrier に上げてよい。
2. carrier は `Stage3PerformHead { op, target_ref = On(target) | Via(chain_ref) }` に留める。
3. request clause suite publicization は entry criteria に保つが、bundle attachment はまだ later に残す。
4. fixture-backed `PerformOn` / `PerformVia` subset comparison と source-side malformed head pair で structural minimum を検証する。
5. diagnostics / final grammar / final public parser-checker-runtime surface にはまだ上げない。

## code anchors

- `crates/mir-ast/src/current_l2.rs`
- `crates/mir-ast/src/lib.rs`
- `crates/mir-ast/tests/current_l2_perform_head_manifest.rs`
- `crates/mir-ast/tests/current_l2_stage3_perform_head_spike.rs`
- `crates/mir-ast/tests/support/current_l2_stage3_predicate_fragment_spike_support.rs`

## stop line

- request_clause_suite_bundle_attachment
- span_rich_diagnostics
- final_grammar
- final_public_parser_checker_runtime_surface

## next promoted line

next promoted line は、
**phase6-perform-head-request-clause-bundle-attachment comparison**
に置く。
