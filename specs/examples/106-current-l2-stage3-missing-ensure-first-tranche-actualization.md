# 106 — current L2 stage 3 missing ensure first tranche actualization

## 目的

この文書は、`specs/examples/105-current-l2-stage3-missing-ensure-vs-request-compare-sequencing.md` で
next narrow step を `missing multiline ensure block` family の actualization に置いたことを前提に、
その hidden fail-closed path が helper-local / test-only actual evidence として
current repo でどこまで surfaced 済みかを示す。

ここで固定するのは final diagnostics carrier でも public parser error API でもない。
固定するのは、fixed two-slot suite bridge helper が current phase で
**どの remaining hidden malformed path を focused smoke として明示済みか**
だけである。

## 前提

- current L2 の core semantics、fixture schema、runtime semantics は変更しない。
- fixed two-slot suite bridge first tranche と first malformed/source pair actualization は already 済みである。
- current helper は private / test-only に留める。
- current tranche では helper code widening は行わない。

## current actualization

current actualization として、
`crates/mir-ast/tests/current_l2_stage3_request_clause_suite_spike.rs`
で次の 1 件を focused smoke に上げてよい。

1. `missing multiline ensure block`

```text
perform write_profile on profile_doc
  ensure:
```

current phase では、これを helper-local wording
`missing multiline predicate block after ensure:`
として substring smoke に留めてよい。

## why this is enough for the current tranche

- current helper が already 持つ hidden fail-closed path を source-backed に surfaced できる。
- helper code 自体の widening を伴わない。
- request head full parse や fixture-side full request contract compare を still later に残せる。

## current code anchor

- helper:
  - `crates/mir-ast/tests/support/current_l2_stage3_request_clause_suite_spike_support.rs`
- tests:
  - `crates/mir-ast/tests/current_l2_stage3_request_clause_suite_spike.rs`

この tranche では helper public surface を広げず、
既存 helper が already 持っていた hidden fail-closed path を
focused smoke として surfaced するだけに留める。

## what did not change

- request head full parse
- fixture-side full request contract compare
- public parser API
- typed diagnostics carrier
- bare clause line family

## evidence boundary

current tranche は helper behavior change ではなく、
**pre-existing hidden fail-closed path の source-backed surfacing**
として読む。

## next narrow step

次は、
**remaining suite malformed family と fixture-side full request contract compare の reopen 条件を改めて比較する**
のが自然である。
