# 0319 — current L2 stage 3 suite malformed first pair actualization

## Objective

Phase 3 mainline として、request-local suite bridge family の first malformed/source pair
`duplicate ensure + unsupported direct child line`
を helper-local / test-only actual evidence として固定する。

## Scope and assumptions

- current L2 core semantics、fixture schema、runtime semantics は変更しない。
- helper は private / test-only のまま維持する。
- current task では public parser API や fixture-side full request compare へ進まない。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `plan/00-index.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/101-current-l2-stage3-request-clause-suite-first-tranche-actualization.md`
- `specs/examples/102-current-l2-stage3-suite-malformed-vs-request-compare-sequencing.md`
- `specs/examples/103-current-l2-stage3-suite-malformed-first-pair-comparison.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`

## Actions taken

1. `current_l2_stage3_request_clause_suite_spike.rs` に duplicate `ensure` と unsupported direct child line の focused smoke test を追加した。
2. helper-local wording が suite bridge family の narrow contract として十分かを targeted test で確認した。
3. current tranche では helper code の behavior change は不要であり、hidden fail-closed path を test-only evidence として surfaced する cut だと `specs/examples/104...` に固定した。

## Evidence / outputs / test results

### Commands run and exact outputs

```text
$ cargo test -p mir-ast --test current_l2_stage3_request_clause_suite_spike
running 8 tests
test stage3_request_clause_suite_spike_allows_ensure_only_suite ... ok
test stage3_request_clause_suite_spike_extracts_single_line_require_and_ensure_slots ... ok
test stage3_request_clause_suite_spike_matches_fixture_fragments_for_mixed_suite ... ok
test stage3_request_clause_suite_spike_rejects_blank_line_between_clauses ... ok
test stage3_request_clause_suite_spike_rejects_duplicate_ensure_clause ... ok
test stage3_request_clause_suite_spike_rejects_duplicate_require_clause ... ok
test stage3_request_clause_suite_spike_rejects_require_after_ensure ... ok
test stage3_request_clause_suite_spike_rejects_unsupported_direct_child_line ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

```text
$ cargo test -p mir-ast
[pass]
```

```text
$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 320 numbered report(s).
```

```text
$ git diff --check
[no output]
```

### Files changed

- [current_l2_stage3_request_clause_suite_spike.rs](/home/yukatayu/dev/mir_poc_01/crates/mir-ast/tests/current_l2_stage3_request_clause_suite_spike.rs)
- [104-current-l2-stage3-suite-malformed-first-pair-actualization.md](/home/yukatayu/dev/mir_poc_01/specs/examples/104-current-l2-stage3-suite-malformed-first-pair-actualization.md)
- [Documentation.md](/home/yukatayu/dev/mir_poc_01/Documentation.md)
- [00-document-map.md](/home/yukatayu/dev/mir_poc_01/specs/00-document-map.md)
- [07-parser-free-poc-stack.md](/home/yukatayu/dev/mir_poc_01/plan/07-parser-free-poc-stack.md)
- [11-roadmap-near-term.md](/home/yukatayu/dev/mir_poc_01/plan/11-roadmap-near-term.md)
- [12-open-problems-and-risks.md](/home/yukatayu/dev/mir_poc_01/plan/12-open-problems-and-risks.md)
- [90-source-traceability.md](/home/yukatayu/dev/mir_poc_01/plan/90-source-traceability.md)
- [progress.md](/home/yukatayu/dev/mir_poc_01/progress.md)

## What changed in understanding

- suite bridge helper は `duplicate ensure` と unsupported direct child line の fail-closed path を already 持っており、current tranche では code widening なしに actual evidence 化できると確認できた。

## Open questions

- unsupported direct child line の wording を current string のまま維持するか、generic unexpected child wording と将来共有するか。
- `missing multiline ensure block` を次段 immediately actualize するか、それとも full request compare comparison の後へ回すか。

## Suggested next prompt

`Phase 3 の次段として、missing multiline ensure block family と fixture-side full request contract compare のどちらを先に開くべきかを narrow に比較してください。`
