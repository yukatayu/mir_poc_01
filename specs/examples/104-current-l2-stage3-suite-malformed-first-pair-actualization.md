# 104 — current L2 stage 3 suite malformed first pair actualization

## 目的

この文書は、`specs/examples/103-current-l2-stage3-suite-malformed-first-pair-comparison.md` で
request-local suite bridge family の first malformed/source pair を
**duplicate `ensure` + unsupported direct child line**
に置くと整理したことを前提に、
その pair が helper-local / test-only actual evidence として
current repo でどこまで actualize 済みかを示す。

ここで固定するのは final diagnostics carrier でも public parser error API でもない。
固定するのは、fixed two-slot suite bridge helper が current phase で
**どの hidden fail-closed path を focused smoke として明示済みか**
だけである。

## 前提

- current L2 の core semantics、fixture schema、runtime semantics は変更しない。
- fixed two-slot suite bridge first tranche は actualize 済みである。
- current stage では helper は private / test-only に留める。
- `missing multiline ensure block` family と fixture-side full request contract compare は still later に残す。

## current actualization

current actualization として、
`crates/mir-ast/tests/current_l2_stage3_request_clause_suite_spike.rs`
で次の 2 件を focused smoke に上げてよい。

1. duplicate `ensure`
2. unsupported direct child line inside fixed two-slot suite

## why this is enough for the current tranche

### duplicate `ensure`

これは already-actualized な duplicate `require` と対になる
at-most-one symmetry evidence である。

```text
perform write_profile on profile_doc
  ensure owner_is(session_user)
  ensure owner_is(session_user)
```

current phase では、これを helper-local wording
`duplicate \`ensure\` clause is not allowed`
として substring smoke に留めてよい。

### unsupported direct child line

これは suite helper が generic continuation parser ではないことを
source-backed に示す evidence である。

```text
perform write_profile on profile_doc
  require write
  note delegated
```

current phase では、これを helper-local wording
`unsupported request-local clause line inside fixed two-slot suite: ...`
として substring smoke に留めてよい。

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
- missing multiline `ensure:` block family

## evidence boundary

current pair actualization は test-first で tests を追加して固定したが、
この tranche では helper code 自体の behavior change は不要である。
つまりここで得た evidence は、
**hidden fail-closed path が already helper に存在していたことを
source-backed に可視化した**
という意味で読む。

## next narrow step

次は、
**missing multiline `ensure:` block family と fixture-side full request contract compare のどちらを先に開くか**
を改めて narrow に比較するのが自然である。
