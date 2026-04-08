# 0313 — current L2 stage 3 request clause suite first tranche actualization

## Objective

Phase 3 mainline として、request-local `require:` / `ensure:` の fixed two-slot suite bridge first tranche を
helper-local / test-only actual evidence として実装し、success-side compare と最小 structural fail-closed smoke を通す。

## Scope and assumptions

- current L2 core semantics、fixture schema、runtime semantics は変更しない。
- stage 3 helper は private / test-only のまま維持する。
- task-start dirty state は `3ac29f4` push 後の clean worktree を前提とする。

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
- `specs/examples/97-current-l2-stage3-multiline-attachment-first-tranche-actualization.md`
- `specs/examples/99-current-l2-stage3-request-clause-suite-structural-floor.md`
- `specs/examples/100-current-l2-stage3-request-clause-suite-first-tranche-comparison.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `crates/mir-ast/tests/support/current_l2_stage3_multiline_attachment_spike_support.rs`
- `crates/mir-ast/tests/support/current_l2_stage3_predicate_fragment_spike_support.rs`

## Actions taken

1. fixed two-slot suite bridge 用 focused test を support file なしで追加し、RED を確認した。
2. `current_l2_stage3_request_clause_suite_spike_support.rs` を追加し、`perform` owner の request-local suite から `require_fragment_text` / `ensure_fragment_text` を抽出する minimal helper を実装した。
3. success-side compare として single-line / mixed multiline / ensure-only suite を fixture-side predicate subset compare へ接続した。
4. structural fail-closed smoke として
   - require-after-ensure
   - duplicate `require`
   - clause-between blank line
   を追加した。
5. actualized scope を `specs/examples/101...` に記録し、mirror を更新した。

## Evidence / outputs / test results

### Commands run and exact outputs

```text
$ cargo test -p mir-ast --test current_l2_stage3_request_clause_suite_spike
   Compiling mir-ast v0.1.0 (/home/yukatayu/dev/mir_poc_01/crates/mir-ast)
error: couldn't read `crates/mir-ast/tests/support/current_l2_stage3_request_clause_suite_spike_support.rs`: No such file or directory (os error 2)
 --> crates/mir-ast/tests/current_l2_stage3_request_clause_suite_spike.rs:3:1
  |
3 | mod current_l2_stage3_request_clause_suite_spike_support;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: could not compile `mir-ast` (test "current_l2_stage3_request_clause_suite_spike") due to 1 previous error
```

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

### Files changed

- [101-current-l2-stage3-request-clause-suite-first-tranche-actualization.md](/home/yukatayu/dev/mir_poc_01/specs/examples/101-current-l2-stage3-request-clause-suite-first-tranche-actualization.md)
- [current_l2_stage3_request_clause_suite_spike_support.rs](/home/yukatayu/dev/mir_poc_01/crates/mir-ast/tests/support/current_l2_stage3_request_clause_suite_spike_support.rs)
- [current_l2_stage3_request_clause_suite_spike.rs](/home/yukatayu/dev/mir_poc_01/crates/mir-ast/tests/current_l2_stage3_request_clause_suite_spike.rs)
- [Documentation.md](/home/yukatayu/dev/mir_poc_01/Documentation.md)
- [00-document-map.md](/home/yukatayu/dev/mir_poc_01/specs/00-document-map.md)
- [07-parser-free-poc-stack.md](/home/yukatayu/dev/mir_poc_01/plan/07-parser-free-poc-stack.md)
- [11-roadmap-near-term.md](/home/yukatayu/dev/mir_poc_01/plan/11-roadmap-near-term.md)
- [12-open-problems-and-risks.md](/home/yukatayu/dev/mir_poc_01/plan/12-open-problems-and-risks.md)
- [90-source-traceability.md](/home/yukatayu/dev/mir_poc_01/plan/90-source-traceability.md)
- [progress.md](/home/yukatayu/dev/mir_poc_01/progress.md)

### Findings

- fixed two-slot suite bridge first tranche は helper-local / test-only helper と focused smoke で actualize してよい。
- `require_fragment_text` / `ensure_fragment_text` carrier により、request-local suite floor を narrow に actualize しつつ existing predicate fragment compare を再利用できる。
- current tranche では duplicate `ensure` や public diagnostics widening を deferred に残しても staged line は壊れない。

## What changed in understanding

- request-local suite floor の first actualization は summary-only helper ではなく clause-slot bridge として切る方が、shared multiline attachment bridge と current predicate helper を滑らかに接続できると確認できた。
- request-local suite malformed family は structural guard と public wideningを分けて進められると分かった。

## Open questions

- duplicate `ensure` と unsupported direct child line を次段 malformed family に入れるか。
- suite bridge の次を request head / full request contract compare に寄せるか、malformed/source extension を先に切るか。

## Suggested next prompt

`Phase 3 の次段として、request-local suite bridge first tranche の後に malformed/source family extension と fixture-side full request contract compare のどちらを先に扱うべきかを整理してください。`
