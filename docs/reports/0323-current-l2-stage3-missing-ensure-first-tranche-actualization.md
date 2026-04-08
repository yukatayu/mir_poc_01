# 0323 — current L2 stage 3 missing ensure first tranche actualization

## Objective

Phase 3 mainline として、request-local suite bridge family の `missing multiline ensure block` hidden path を
helper-local / test-only actual evidence として固定する。

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
- `specs/examples/104-current-l2-stage3-suite-malformed-first-pair-actualization.md`
- `specs/examples/105-current-l2-stage3-missing-ensure-vs-request-compare-sequencing.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`

## Actions taken

1. `current_l2_stage3_request_clause_suite_spike.rs` に `missing multiline ensure block` の focused smoke test を追加した。
2. helper-local wording が current suite bridge family の narrow contract として十分かを targeted test で確認した。
3. current tranche では helper code の behavior change は不要であり、hidden fail-closed path を test-only evidence として surfaced する cut だと `specs/examples/106...` に固定した。

## Evidence / outputs / test results

### Commands run and exact outputs

```text
$ cargo test -p mir-ast --test current_l2_stage3_request_clause_suite_spike
running 9 tests
...
test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

```text
$ cargo test -p mir-ast
[pass]
```

```text
$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 326 numbered report(s).
```

```text
$ git diff --check
[no output]
```

### Files changed

- [current_l2_stage3_request_clause_suite_spike.rs](/home/yukatayu/dev/mir_poc_01/crates/mir-ast/tests/current_l2_stage3_request_clause_suite_spike.rs)
- [106-current-l2-stage3-missing-ensure-first-tranche-actualization.md](/home/yukatayu/dev/mir_poc_01/specs/examples/106-current-l2-stage3-missing-ensure-first-tranche-actualization.md)
- [Documentation.md](/home/yukatayu/dev/mir_poc_01/Documentation.md)
- [00-document-map.md](/home/yukatayu/dev/mir_poc_01/specs/00-document-map.md)
- [90-source-traceability.md](/home/yukatayu/dev/mir_poc_01/plan/90-source-traceability.md)
- [progress.md](/home/yukatayu/dev/mir_poc_01/progress.md)

## What changed in understanding

- suite bridge helper は `missing multiline ensure block` の fail-closed path を already 持っており、current tranche では code widening なしに actual evidence 化できると確認できた。

## Open questions

- `missing multiline ensure block` wording を current string のまま維持するか、later generic multiline family と共有するか。
- next task の comparison では、remaining suite malformed wording family に bare clause line family を含めるかどうかを明示的に切り分ける必要がある。

## Suggested next prompt

`Phase 3 の次段として、remaining suite malformed family と fixture-side full request contract compare の reopen 条件を narrow に比較してください。`
