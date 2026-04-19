# 562 — current L2 phase6-request-clause-suite-publicization threshold helper mirror

## 目的

`specs/examples/99-current-l2-stage3-request-clause-suite-structural-floor.md`、
`100-current-l2-stage3-request-clause-suite-first-tranche-comparison.md`、
`101-current-l2-stage3-request-clause-suite-first-tranche-actualization.md`
で source-backed に積んだ request-local fixed two-slot suite line を、
`mir_ast::current_l2` の crate-local non-production carrier に narrow に映す。

ここで固定するのは
**current L2 phase6-request-clause-suite-publicization minimum**
であり、

- perform head final public parser API
- span-rich diagnostics
- final grammar
- final public parser / checker / runtime surface

はまだ固定しない。

## helper mirror shape

```text
phase6_request_clause_suite_publicization = {
  carrier_kind = current_l2_nonproduction_request_clause_suite_carrier,
  accepted_surface_refs = [
    stage3_request_clause_suite_surface,
    stage3_request_clause_multiline_extraction_surface,
    stage3_minimal_predicate_fragment_surface
  ],
  code_anchor_refs = [
    mir_ast_current_l2_module,
    stage3_request_clause_suite_tests
  ],
  retained_later_refs = [
    perform_head_final_public_parser_api,
    span_rich_diagnostics,
    final_grammar
  ]
}

request_clause_suite_bridge = {
  carrier = Stage3RequestClauseSuite {
    require_fragment_text,
    ensure_fragment_text
  },
  owner = perform,
  ordering = require_then_ensure,
  multiplicity = at_most_one_each,
  termination = dedent_or_next_statement_head,
  blank_line_between_clauses = reject
}
```

## current actualization cut

- `crates/mir-ast/src/current_l2.rs` に
  - `CurrentL2RequestClauseSuiteManifest`
  - `current_l2_request_clause_suite_manifest()`
  - `Stage3RequestClauseSuite`
  - `parse_stage3_request_clause_suite_text()`
  を置き、request clause suite line の minimum を crate 本体で inspectable にする。
- accepted surface は、
  - fixed two-slot suite bridge
  - shared multiline extraction reuse
  - existing predicate fragment parser reuse
  に留める。
- helper-local structural guard として
  - `require` after `ensure`
  - duplicate `require`
  - duplicate `ensure`
  - clause-between blank line
  - missing multiline `ensure:` block
  - unsupported direct child line
  を current suite carrier と一緒に drift なく保つ。
- `crates/mir-ast/tests/current_l2_stage3_request_clause_suite_spike.rs`
  は support-local duplicate implementation をやめ、
  crate-local carrier を直接検証する。

## practical reading

current helper mirror が示すのは、次の 5 点だけである。

1. request-local `require` / `ensure` suite は non-production parser carrier に上げてよい。
2. carrier は `Stage3RequestClauseSuite { require_fragment_text, ensure_fragment_text }` の two-slot bundle に留める。
3. suite owner は `perform`、ordering は `require` の後に `ensure`、multiplicity は各 at-most-one に留める。
4. multiline clause payload extraction は shared single attachment frame と predicate fragment parser reuse に寄せる。
5. perform head final public parser API / diagnostics / final grammar にはまだ上げない。

## code anchors

- `crates/mir-ast/src/current_l2.rs`
- `crates/mir-ast/src/lib.rs`
- `crates/mir-ast/tests/current_l2_request_clause_suite_manifest.rs`
- `crates/mir-ast/tests/current_l2_stage3_request_clause_suite_spike.rs`

## stop line

- perform_head_final_public_parser_api
- span_rich_diagnostics
- final_grammar
- final_public_parser_checker_runtime_surface

## next promoted line

next promoted line は、
**phase6-perform-head-final-public-parser-api comparison**
に置く。
