# 564 — current L2 phase6-perform-head-request-clause-bundle-attachment comparison

## 目的

`562-current-l2-phase6-request-clause-suite-publicization-threshold-helper-mirror.md`
と
`563-current-l2-phase6-perform-head-structural-carrier-threshold-helper-mirror.md`
で separate carrier として narrow に actualize した

- request-local fixed two-slot clause suite
- `perform` head owner / op / target-or-via shape

を、どの combined carrier で helper-local / test-only に束ねるべきか比較する。

ここで固定するのは
**current L2 phase6-perform-head-request-clause-bundle-attachment compare floor**
であり、

- final parser grammar
- final public parser / checker / runtime API
- full `Program` lowering
- richer predicate grammar
- span-rich diagnostics

はまだ固定しない。

## current first comparison choice

current first comparison choice は、flatten でも generic attachment table でもなく、
**request-local two-slot suite を head bundle に薄く差す** shape に置く。

```text
phase6_perform_head_request_clause_bundle_attachment = {
  carrier_kind = current_l2_nonproduction_request_head_clause_bundle,
  input_refs = [
    phase6_request_clause_suite_publicization,
    phase6_perform_head_structural_carrier
  ],
  accepted_shape_ref = request_local_two_slot_suite_bundle,
  retained_later_refs = [
    span_rich_diagnostics,
    final_grammar,
    final_public_parser_checker_runtime_surface,
    full_program_lowering
  ]
}

request_head_clause_bundle = Stage3RequestHeadClauseBundle {
  perform_head: Stage3PerformHead,
  clause_suite: Stage3RequestClauseSuite,
  attachment_frame_kind: RequestLocalTwoSlotSuite,
}
```

## retained alternatives

### alternative A — flatten clause suite into `Stage3PerformHead`

- current reading:
  今は採らない。
- reason:
  head shape と clause-suite attachment の責務が collapse しやすく、
  Package 89 / 90 で切った separate minimum cut を壊しやすい。

### alternative B — generic stringly attachment frame first

- current reading:
  今は採らない。
- reason:
  helper-local compare floor としては guard が弱く、
  request-local fixed two-slot suite という current accepted cut を不用意に曖昧化する。

## practical reading

current compare floor が意味するのは次の 5 点である。

1. request clause suite と perform head は separate carrier minimum を保ったまま combined carrier を比較してよい。
2. first combined carrier は `Stage3RequestHeadClauseBundle` のような thin wrapper でよい。
3. accepted attachment frame は request-local fixed two-slot suite に留める。
4. malformed attachment は fail-closed に保ち、head minimum / suite minimum を separately に再利用できる形を優先する。
5. diagnostics / final grammar / final public parser-checker-runtime surface にはまだ上げない。

## code anchors

- `crates/mir-ast/src/current_l2.rs`
- `crates/mir-ast/tests/current_l2_request_clause_suite_manifest.rs`
- `crates/mir-ast/tests/current_l2_perform_head_manifest.rs`

## stop line

- span-rich diagnostics
- final grammar
- final public parser / checker / runtime surface
- full `Program` lowering

## next promoted line

Package 91 close 後の next promoted line は、
**Package 92 — first strong typing finite-index layer**
に置く。
