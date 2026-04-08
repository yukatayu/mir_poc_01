# 0314 — review current L2 stage 3 request clause suite first tranche actualization

## Objective

`0313-current-l2-stage3-request-clause-suite-first-tranche-actualization.md` と対応する code / spec / mirror change が、
Phase 3 mainline の staged line と current helper boundary を壊していないかを確認する。

## Scope and assumptions

- current L2 core semantics は変更しない。
- review は fixed two-slot suite bridge first tranche の helper-local / test-only actualization に限定する。

## Documents consulted

- `docs/reports/0313-current-l2-stage3-request-clause-suite-first-tranche-actualization.md`
- `specs/examples/100-current-l2-stage3-request-clause-suite-first-tranche-comparison.md`
- `specs/examples/101-current-l2-stage3-request-clause-suite-first-tranche-actualization.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `progress.md`

## Actions taken

1. `specs/examples/100...` と `specs/examples/101...` を突き合わせ、helper output が fixed two-slot suite bridge の slot carrierに留まり、fixture-side full request contract compare へ滑っていないかを見直した。
2. `crates/mir-ast/tests/support/current_l2_stage3_request_clause_suite_spike_support.rs` と `crates/mir-ast/tests/current_l2_stage3_request_clause_suite_spike.rs` を読み、success-side compare と structural fail-closed smoke が narrow first tranche に収まっているかを確認した。
3. reviewer completion channel はこの session で使えなかったため、local diff inspection と validation evidence による fallback review を行った。

## Evidence / outputs / test results

### Commands run and exact outputs

```text
$ cargo test -p mir-ast --test current_l2_stage3_request_clause_suite_spike
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.57s
     Running tests/current_l2_stage3_request_clause_suite_spike.rs (target/debug/deps/current_l2_stage3_request_clause_suite_spike-6bbab3b51fcdbd25)

running 6 tests
test stage3_request_clause_suite_spike_extracts_single_line_require_and_ensure_slots ... ok
test stage3_request_clause_suite_spike_allows_ensure_only_suite ... ok
test stage3_request_clause_suite_spike_rejects_blank_line_between_clauses ... ok
test stage3_request_clause_suite_spike_rejects_duplicate_require_clause ... ok
test stage3_request_clause_suite_spike_matches_fixture_fragments_for_mixed_suite ... ok
test stage3_request_clause_suite_spike_rejects_require_after_ensure ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

```text
$ cargo test -p mir-ast
[pass: unit 0 / integration 32 / doc-tests 0]
```

### Findings

- substantive finding はない。
- helper output は `require_fragment_text` / `ensure_fragment_text` の two-slot carrier に留まっており、request head parse や fixture-side full request contract compare を hidden に導入していない。
- success-side tests は single-line / multiline / ensure-only の 3 系統に留まり、narrow first tranche として十分である。
- fail-closed smoke も `require` after `ensure`、duplicate `require`、clause-between blank line の最小 structural guard に収まっている。

## What changed in understanding

- fixed two-slot suite bridge first tranche は、summary-only helper よりも existing predicate fragment helper との接続が素直であり、それでも helper boundary は still narrow だと確認できた。
- 次段では helper-local malformed/source family を広げるか、fixture-side full request contract compare に近づく structural compare を先に切るかの sequencing が主論点になる。

## Open questions

- duplicate `ensure` と unsupported direct child line を、first-tranche follow-up の malformed/source family に入れるか。
- suite bridge helper の structural fail-closed をどこまで docs/report で独立 family として扱うか。

## Suggested next prompt

`Phase 3 の次段として、request-local suite bridge first tranche の後に malformed/source family extension と fixture-side full request contract compare のどちらを先に扱うべきかを整理してください。`
