# 0299 — current L2 stage 3 predicate fragment first tranche

## Objective

Phase 3 mainline として、`specs/examples/92-current-l2-stage3-predicate-fragment-reopen-sequencing.md`
の次段である predicate fragment boundary を narrow に actualize する。

## Scope and assumptions

- current L2 の core semantics、fixture schema、runtime semantics、detached validation loop の core は変更しない。
- stage 3 helper は private / test-only に留める。
- final parser grammar、request head + clause attachment multiline shape、fixture-side full request node compare は扱わない。
- task-start dirty state は、この task の前段 closeout 直後の clean worktree を前提とする。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/01-current-l2-surface-syntax-candidates.md`
- `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md`
- `specs/examples/89-current-l2-stage3-admit-node-handoff-comparison.md`
- `specs/examples/90-current-l2-stage3-request-local-clause-spillover-comparison.md`
- `specs/examples/91-current-l2-stage3-request-local-clause-spillover-first-tranche-actualization.md`
- `specs/examples/92-current-l2-stage3-predicate-fragment-reopen-sequencing.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `crates/mir-ast/tests/support/current_l2_stage3_admit_slot_spike_support.rs`
- `crates/mir-ast/tests/current_l2_stage3_admit_slot_spike.rs`
- `crates/mir-ast/tests/fixtures/current-l2/e2-try-fallback.json`
- `crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json`
- `crates/mir-ast/tests/fixtures/current-l2/e10-perform-on-ensure-failure.json`
- `crates/mir-ast/tests/fixtures/current-l2/e11-perform-via-ensure-then-success.json`

## Actions taken

1. stage 3 later branch の next step として、predicate fragment reopen の exact cut を docs-only で比較した。
2. shared isolated predicate fragment helper を first choice にし、request head attachment を still later stage に残す judgment を [93-current-l2-stage3-predicate-fragment-boundary-comparison.md](/home/yukatayu/dev/mir_poc_01/specs/examples/93-current-l2-stage3-predicate-fragment-boundary-comparison.md) に固定した。
3. test-first で [current_l2_stage3_predicate_fragment_spike.rs](/home/yukatayu/dev/mir_poc_01/crates/mir-ast/tests/current_l2_stage3_predicate_fragment_spike.rs) を追加し、missing support file で red を確認した。
4. [current_l2_stage3_predicate_fragment_spike_support.rs](/home/yukatayu/dev/mir_poc_01/crates/mir-ast/tests/support/current_l2_stage3_predicate_fragment_spike_support.rs) を追加し、isolated fragment parser と fixture-side predicate subset loader を actualize した。
5. first tranche actualization を [94-current-l2-stage3-predicate-fragment-first-tranche-actualization.md](/home/yukatayu/dev/mir_poc_01/specs/examples/94-current-l2-stage3-predicate-fragment-first-tranche-actualization.md) に記録し、mirror / progress / traceability を更新した。

## Evidence / outputs / test results

### Commands run and exact outputs

```text
$ date --iso-8601=seconds
2026-04-08T13:50:40+09:00
```

```text
$ cargo test -p mir-ast --test current_l2_stage3_predicate_fragment_spike
error: couldn't read `crates/mir-ast/tests/support/current_l2_stage3_predicate_fragment_spike_support.rs`: No such file or directory (os error 2)
 --> crates/mir-ast/tests/current_l2_stage3_predicate_fragment_spike.rs:2:1
  |
2 | mod current_l2_stage3_predicate_fragment_spike_support;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: could not compile `mir-ast` (test "current_l2_stage3_predicate_fragment_spike") due to 1 previous error
```

```text
$ cargo test -p mir-ast --test current_l2_stage3_predicate_fragment_spike
running 7 tests
test stage3_predicate_fragment_spike_keeps_explicit_and_terms ... ok
test stage3_predicate_fragment_spike_matches_e10_request_local_require ... ok
test stage3_predicate_fragment_spike_matches_e10_request_local_ensure ... ok
test stage3_predicate_fragment_spike_matches_e3_delegated_writer_admit ... ok
test stage3_predicate_fragment_spike_matches_e3_owner_writer_admit ... ok
test stage3_predicate_fragment_spike_matches_e11_request_local_ensure ... ok
test stage3_predicate_fragment_spike_matches_grouped_and_fixture_fragment ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

```text
$ cargo test -p mir-ast
test result: ok. 18 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

```text
$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 300 numbered report(s).
```

```text
$ git diff --check
[no output]
```

### Files changed

- [93-current-l2-stage3-predicate-fragment-boundary-comparison.md](/home/yukatayu/dev/mir_poc_01/specs/examples/93-current-l2-stage3-predicate-fragment-boundary-comparison.md)
- [94-current-l2-stage3-predicate-fragment-first-tranche-actualization.md](/home/yukatayu/dev/mir_poc_01/specs/examples/94-current-l2-stage3-predicate-fragment-first-tranche-actualization.md)
- [current_l2_stage3_predicate_fragment_spike_support.rs](/home/yukatayu/dev/mir_poc_01/crates/mir-ast/tests/support/current_l2_stage3_predicate_fragment_spike_support.rs)
- [current_l2_stage3_predicate_fragment_spike.rs](/home/yukatayu/dev/mir_poc_01/crates/mir-ast/tests/current_l2_stage3_predicate_fragment_spike.rs)
- [Documentation.md](/home/yukatayu/dev/mir_poc_01/Documentation.md)
- [00-document-map.md](/home/yukatayu/dev/mir_poc_01/specs/00-document-map.md)
- [07-parser-free-poc-stack.md](/home/yukatayu/dev/mir_poc_01/plan/07-parser-free-poc-stack.md)
- [11-roadmap-near-term.md](/home/yukatayu/dev/mir_poc_01/plan/11-roadmap-near-term.md)
- [12-open-problems-and-risks.md](/home/yukatayu/dev/mir_poc_01/plan/12-open-problems-and-risks.md)
- [90-source-traceability.md](/home/yukatayu/dev/mir_poc_01/plan/90-source-traceability.md)
- [progress.md](/home/yukatayu/dev/mir_poc_01/progress.md)

## What changed in understanding

- predicate fragment boundary は `decl_admit_slot.surface_text` retention の延長ではなく、declaration-side `admit` と request-local `require` / `ensure` の **shared floor** として切る方が理論的にきれいだと確認できた。
- その shared floor は full program parser を広げなくても、isolated fragment helper と fixture-side predicate subset compare だけで actual evidence を積める。
- `e2` / `e3` / `e10` / `e11` を working set にすると、
  - bare atom
  - application-like form
  - explicit `and`
  - grouping
  の current minimal predicate fragment family を source-backed に覆える。

## Open questions

- next narrow step は、predicate fragment helper の malformed-source pair を先に actualize するか、それとも request head + clause attachment multiline shape の docs-only comparison を先に開くか。
- declaration-side `admit:` / request-local `require:` / `ensure:` の multiline block extraction を、isolated fragment helper と program parser accepted cluster のどちらに先に載せるかはまだ OPEN。
- helper-local fragment parser の wording fragment を将来 public diagnostics surface に昇格させるかは未決であり、current phase では private / test-only に留める。

## Suggested next prompt

`Phase 3 の次段として、stage 3 predicate fragment helper の malformed-source first tranche と request head + clause attachment multiline shape の sequencing を比較してください。`
