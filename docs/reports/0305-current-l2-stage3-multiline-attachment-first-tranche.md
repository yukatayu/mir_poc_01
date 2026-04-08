# 0305 — current L2 stage 3 multiline attachment first tranche

## Objective

Phase 3 mainline として、shared single attachment frame の helper-local / test-only first tranche を actualize し、
declaration-side `admit:` と request-local `require:` / `ensure:` が同じ multiline attachment bridge を共有できるかを確認する。

## Scope and assumptions

- current L2 core semantics、fixture schema、runtime semantics は変更しない。
- stage 3 helper は private / test-only に留める。
- full parser actualization や public parser API 化は行わない。
- task-start dirty state は `9fed05f` push 後の clean worktree を前提とする。

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
- `specs/examples/01-current-l2-surface-syntax-candidates.md`
- `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md`
- `specs/examples/94-current-l2-stage3-predicate-fragment-first-tranche-actualization.md`
- `specs/examples/96-current-l2-stage3-multiline-attachment-shape-comparison.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`

## Actions taken

1. missing support file を参照する `current_l2_stage3_multiline_attachment_spike.rs` を先に追加し、targeted test が file-missing で fail することを確認した。
2. `current_l2_stage3_multiline_attachment_spike_support.rs` を追加し、declaration-side `admit:` と request-local `require:` / `ensure:` の multiline block extraction を helper-local に実装した。
3. reviewer finding を受けて、clause header search を immediate child attachment line に制限し、blank line を helper-local で fail-closed に reject する tightening を追加した。
4. extracted fragment source を既存の shared isolated predicate fragment helper に渡し、grouped `and` / single-line call の compare と missing-block / nested-header / blank-line malformed smoke を通した。
5. actualization scope を `specs/examples/97...` に固定し、mirror / progress を更新した。

## Evidence / outputs / test results

### Commands run and exact outputs

```text
$ date '+%Y-%m-%d %H:%M %Z'
2026-04-08 14:30 JST
```

```text
$ cargo test -p mir-ast --test current_l2_stage3_multiline_attachment_spike
   Compiling mir-ast v0.1.0 (/home/yukatayu/dev/mir_poc_01/crates/mir-ast)
error: couldn't read `crates/mir-ast/tests/support/current_l2_stage3_multiline_attachment_spike_support.rs`: No such file or directory (os error 2)
 --> crates/mir-ast/tests/current_l2_stage3_multiline_attachment_spike.rs:2:1
  |
2 | mod current_l2_stage3_multiline_attachment_spike_support;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

```text
$ cargo test -p mir-ast --test current_l2_stage3_multiline_attachment_spike
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.08s
     Running tests/current_l2_stage3_multiline_attachment_spike.rs (target/debug/deps/current_l2_stage3_multiline_attachment_spike-36b0d35e7400751b)

running 8 tests
test stage3_multiline_attachment_spike_extracts_request_local_ensure_fragment ... ok
test stage3_multiline_attachment_spike_extracts_option_local_admit_fragment ... ok
test stage3_multiline_attachment_spike_extracts_request_local_require_fragment ... ok
test stage3_multiline_attachment_spike_keeps_grouped_and_structure ... ok
test stage3_multiline_attachment_spike_rejects_blank_line_inside_block ... FAILED
test stage3_multiline_attachment_spike_rejects_missing_admit_block_payload ... ok
test stage3_multiline_attachment_spike_rejects_missing_require_block_payload ... ok
test stage3_multiline_attachment_spike_rejects_nested_clause_header_search ... FAILED

failures:
---- stage3_multiline_attachment_spike_rejects_blank_line_inside_block stdout ----
multiline attachment spike should reject blank lines inside block: \"owner_is(session_user)\\nwell_formed(profile_draft)\"
---- stage3_multiline_attachment_spike_rejects_nested_clause_header_search stdout ----
multiline attachment spike should not search nested clause-like lines: \"owner_is(session_user)\"
test result: FAILED. 6 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
error: test failed, to rerun pass `-p mir-ast --test current_l2_stage3_multiline_attachment_spike`
```

```text
$ cargo test -p mir-ast --test current_l2_stage3_multiline_attachment_spike
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.40s
     Running tests/current_l2_stage3_multiline_attachment_spike.rs (target/debug/deps/current_l2_stage3_multiline_attachment_spike-36b0d35e7400751b)

running 8 tests
test stage3_multiline_attachment_spike_extracts_request_local_ensure_fragment ... ok
test stage3_multiline_attachment_spike_extracts_option_local_admit_fragment ... ok
test stage3_multiline_attachment_spike_extracts_request_local_require_fragment ... ok
test stage3_multiline_attachment_spike_keeps_grouped_and_structure ... ok
test stage3_multiline_attachment_spike_rejects_blank_line_inside_block ... ok
test stage3_multiline_attachment_spike_rejects_missing_admit_block_payload ... ok
test stage3_multiline_attachment_spike_rejects_missing_require_block_payload ... ok
test stage3_multiline_attachment_spike_rejects_nested_clause_header_search ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

```text
$ cargo test -p mir-ast
warning: function `load_fixture_option_admit_fragment` is never used
warning: function `find_option_decl` is never used
warning: `mir-ast` (test "current_l2_stage3_multiline_attachment_spike") generated 2 warnings
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running unittests src/lib.rs (target/debug/deps/mir_ast-415d708894b93c1e)
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
     Running tests/current_l2_stage1_parser_spike.rs (target/debug/deps/current_l2_stage1_parser_spike-1b76a4c12fa11c85)
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
     Running tests/current_l2_stage3_admit_slot_spike.rs (target/debug/deps/current_l2_stage3_admit_slot_spike-3ca18148dd0b1479)
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
     Running tests/current_l2_stage3_multiline_attachment_spike.rs (target/debug/deps/current_l2_stage3_multiline_attachment_spike-36b0d35e7400751b)
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
     Running tests/current_l2_stage3_predicate_fragment_spike.rs (target/debug/deps/current_l2_stage3_predicate_fragment_spike-398bcfac941161ca)
test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
   Doc-tests mir_ast
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

```text
$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 306 numbered report(s).
```

```text
$ git diff --check
[no output]
```

### Files changed

- [97-current-l2-stage3-multiline-attachment-first-tranche-actualization.md](/home/yukatayu/dev/mir_poc_01/specs/examples/97-current-l2-stage3-multiline-attachment-first-tranche-actualization.md)
- [current_l2_stage3_multiline_attachment_spike_support.rs](/home/yukatayu/dev/mir_poc_01/crates/mir-ast/tests/support/current_l2_stage3_multiline_attachment_spike_support.rs)
- [current_l2_stage3_multiline_attachment_spike.rs](/home/yukatayu/dev/mir_poc_01/crates/mir-ast/tests/current_l2_stage3_multiline_attachment_spike.rs)
- [Documentation.md](/home/yukatayu/dev/mir_poc_01/Documentation.md)
- [00-document-map.md](/home/yukatayu/dev/mir_poc_01/specs/00-document-map.md)
- [07-parser-free-poc-stack.md](/home/yukatayu/dev/mir_poc_01/plan/07-parser-free-poc-stack.md)
- [11-roadmap-near-term.md](/home/yukatayu/dev/mir_poc_01/plan/11-roadmap-near-term.md)
- [12-open-problems-and-risks.md](/home/yukatayu/dev/mir_poc_01/plan/12-open-problems-and-risks.md)
- [90-source-traceability.md](/home/yukatayu/dev/mir_poc_01/plan/90-source-traceability.md)
- [progress.md](/home/yukatayu/dev/mir_poc_01/progress.md)

### Findings

- shared single attachment frame は helper-local / test-only actual evidence として十分 narrow に actualize できる。
- declaration-side `admit:` と request-local `require:` / `ensure:` は、full parser actualization を行わなくても multiline block extraction bridge を共有できる。
- current first tranche では、`admit:` と `require:` の missing block に加えて、nested header over-acceptance を防ぐ missing-header check と blank-line fail-closed guard まで入れるのが boundary に忠実だった。

## What changed in understanding

- multiline attachment comparison は docs-only judgment に留まらず、isolated fragment helper へつながる source carrier bridge として actualize できることが確認できた。
- 次段の主要論点は attachment frame そのものではなく、request-local clause suite completion と malformed-source pair の sequencing に移った。

## Open questions

- request-local clause suite ordering / multiplicity / dedent completion を、次段 docs-only comparison でどこまで切るか。
- multiline attachment malformed-source pair を増やすなら、`ensure:` missing block と request-local suite completion malformed family のどちらを先に扱うか。

## Suggested next prompt

`Phase 3 の次段として、request-local clause suite completion と multiline attachment malformed-source pair のどちらを先に扱うべきかを整理してください。`
